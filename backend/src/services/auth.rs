use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    config::Config,
    models::{
        AuthResponse, LoginDto, PasswordResetDto, PasswordResetRequestDto, RefreshToken,
        RefreshTokenDto, RegisterDto, User, UserResponse,
    },
    utils::{AppError, Result},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,     // user_id
    pub email: String,
    pub exp: i64,      // expiration timestamp
    pub iat: i64,      // issued at timestamp
}

pub struct AuthService;

impl AuthService {
    pub async fn register(
        db: &PgPool,
        dto: RegisterDto,
    ) -> Result<AuthResponse> {
        // Check if user already exists
        let existing = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM users WHERE email = $1"
        )
        .bind(&dto.email)
        .fetch_one(db)
        .await?;

        if existing > 0 {
            return Err(AppError::BadRequest("Email already registered".to_string()));
        }

        // Hash password
        let password_hash = Self::hash_password(&dto.password)?;

        // Create user
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (email, password_hash, display_name, email_verified)
            VALUES ($1, $2, $3, false)
            RETURNING *
            "#
        )
        .bind(&dto.email)
        .bind(&password_hash)
        .bind(&dto.display_name)
        .fetch_one(db)
        .await?;

        // Generate tokens
        let config = Config::from_env().map_err(|e| AppError::ConfigError(e.to_string()))?;
        let (access_token, refresh_token) = Self::generate_tokens(&user, &config, db).await?;

        Ok(AuthResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: config.jwt.expiration,
            user: Self::user_to_response(&user),
        })
    }

    pub async fn login(
        db: &PgPool,
        dto: LoginDto,
    ) -> Result<AuthResponse> {
        // Find user
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(&dto.email)
        .fetch_optional(db)
        .await?
        .ok_or(AppError::Unauthorized)?;

        // Verify password
        if !Self::verify_password(&dto.password, &user.password_hash)? {
            // Record failed login attempt
            Self::record_login_attempt(db, &dto.email, None, false).await?;
            return Err(AppError::Unauthorized);
        }

        // Record successful login attempt
        Self::record_login_attempt(db, &dto.email, Some(user.id), true).await?;

        // Generate tokens
        let config = Config::from_env().map_err(|e| AppError::ConfigError(e.to_string()))?;
        let (access_token, refresh_token) = Self::generate_tokens(&user, &config, db).await?;

        Ok(AuthResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: config.jwt.expiration,
            user: Self::user_to_response(&user),
        })
    }

    pub async fn refresh_token(
        db: &PgPool,
        dto: RefreshTokenDto,
    ) -> Result<AuthResponse> {
        // Find and validate refresh token
        let token_record = sqlx::query_as::<_, RefreshToken>(
            r#"
            SELECT * FROM refresh_tokens 
            WHERE token = $1 
                AND revoked_at IS NULL 
                AND expires_at > NOW()
            "#
        )
        .bind(&dto.refresh_token)
        .fetch_optional(db)
        .await?
        .ok_or(AppError::Unauthorized)?;

        // Get user
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(token_record.user_id)
        .fetch_optional(db)
        .await?
        .ok_or(AppError::Unauthorized)?;

        // Revoke old refresh token
        sqlx::query(
            "UPDATE refresh_tokens SET revoked_at = NOW() WHERE id = $1"
        )
        .bind(token_record.id)
        .execute(db)
        .await?;

        // Generate new tokens
        let config = Config::from_env().map_err(|e| AppError::ConfigError(e.to_string()))?;
        let (access_token, refresh_token) = Self::generate_tokens(&user, &config, db).await?;

        Ok(AuthResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: config.jwt.expiration,
            user: Self::user_to_response(&user),
        })
    }

    pub async fn logout(db: &PgPool, user_id: Uuid) -> Result<()> {
        // Revoke all refresh tokens for user
        sqlx::query(
            "UPDATE refresh_tokens SET revoked_at = NOW() WHERE user_id = $1 AND revoked_at IS NULL"
        )
        .bind(user_id)
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn request_password_reset(
        db: &PgPool,
        dto: PasswordResetRequestDto,
    ) -> Result<()> {
        // Find user
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(&dto.email)
        .fetch_optional(db)
        .await?;

        // Always return success to prevent email enumeration
        if let Some(user) = user {
            // Generate reset token
            let token = Self::generate_random_token();
            let expires_at = Utc::now() + Duration::hours(1);

            // Store token
            sqlx::query(
                r#"
                INSERT INTO password_reset_tokens (user_id, token, expires_at)
                VALUES ($1, $2, $3)
                "#
            )
            .bind(user.id)
            .bind(&token)
            .bind(expires_at)
            .execute(db)
            .await?;

            // TODO: Send email with reset link
            tracing::info!("Password reset token generated for user {}: {}", user.email, token);
        }

        Ok(())
    }

    pub async fn reset_password(
        db: &PgPool,
        dto: PasswordResetDto,
    ) -> Result<()> {
        // Find valid token
        let token_record = sqlx::query!(
            r#"
            SELECT user_id FROM password_reset_tokens 
            WHERE token = $1 
                AND used_at IS NULL 
                AND expires_at > NOW()
            "#,
            dto.token
        )
        .fetch_optional(db)
        .await?
        .ok_or(AppError::BadRequest("Invalid or expired token".to_string()))?;

        // Hash new password
        let password_hash = Self::hash_password(&dto.new_password)?;

        // Update password
        sqlx::query(
            "UPDATE users SET password_hash = $1, updated_at = NOW() WHERE id = $2"
        )
        .bind(&password_hash)
        .bind(token_record.user_id)
        .execute(db)
        .await?;

        // Mark token as used
        sqlx::query(
            "UPDATE password_reset_tokens SET used_at = NOW() WHERE token = $1"
        )
        .bind(&dto.token)
        .execute(db)
        .await?;

        // Revoke all refresh tokens
        sqlx::query(
            "UPDATE refresh_tokens SET revoked_at = NOW() WHERE user_id = $1 AND revoked_at IS NULL"
        )
        .bind(token_record.user_id)
        .execute(db)
        .await?;

        Ok(())
    }

    // Helper methods
    async fn generate_tokens(
        user: &User,
        config: &Config,
        db: &PgPool,
    ) -> Result<(String, String)> {
        // Generate access token
        let access_token = Self::generate_jwt(user, config)?;

        // Generate refresh token
        let refresh_token = Self::generate_random_token();
        let expires_at = Utc::now() + Duration::days(30);

        // Store refresh token
        sqlx::query(
            r#"
            INSERT INTO refresh_tokens (user_id, token, expires_at)
            VALUES ($1, $2, $3)
            "#
        )
        .bind(user.id)
        .bind(&refresh_token)
        .bind(expires_at)
        .execute(db)
        .await?;

        Ok((access_token, refresh_token))
    }

    fn generate_jwt(user: &User, config: &Config) -> Result<String> {
        let expiration = Utc::now() + Duration::seconds(config.jwt.expiration);
        
        let claims = Claims {
            sub: user.id,
            email: user.email.clone(),
            exp: expiration.timestamp(),
            iat: Utc::now().timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(config.jwt.secret.as_bytes()),
        )
        .map_err(|_e| AppError::InternalServerError)?;

        Ok(token)
    }

    pub fn validate_jwt(token: &str, config: &Config) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(config.jwt.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized)?;

        Ok(token_data.claims)
    }

    fn hash_password(password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| AppError::InternalServerError)?
            .to_string();

        Ok(password_hash)
    }

    fn verify_password(password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|_| AppError::InternalServerError)?;
        
        let argon2 = Argon2::default();
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }

    fn generate_random_token() -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let token: String = (0..32)
            .map(|_| {
                let idx = rng.gen_range(0..62);
                match idx {
                    0..=9 => (b'0' + idx) as char,
                    10..=35 => (b'a' + idx - 10) as char,
                    36..=61 => (b'A' + idx - 36) as char,
                    _ => unreachable!(),
                }
            })
            .collect();
        token
    }

    fn user_to_response(user: &User) -> UserResponse {
        UserResponse {
            id: user.id,
            email: user.email.clone(),
            display_name: user.display_name.clone(),
            email_verified: user.email_verified,
            created_at: user.created_at,
        }
    }

    async fn record_login_attempt(
        db: &PgPool,
        email: &str,
        user_id: Option<Uuid>,
        success: bool,
    ) -> Result<()> {
        // In a real application, you'd get the IP from the request
        let ip_address = "127.0.0.1";
        
        sqlx::query(
            r#"
            INSERT INTO login_attempts (email, ip_address, success)
            VALUES ($1, $2::inet, $3)
            "#
        )
        .bind(email)
        .bind(ip_address)
        .bind(success)
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn check_rate_limit(db: &PgPool, email: &str) -> Result<()> {
        let attempts = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM login_attempts
            WHERE email = $1 
                AND attempted_at > NOW() - INTERVAL '15 minutes'
                AND success = false
            "#
        )
        .bind(email)
        .fetch_one(db)
        .await?;

        if attempts >= 5 {
            return Err(AppError::BadRequest("Too many login attempts. Please try again later.".to_string()));
        }

        Ok(())
    }
}
