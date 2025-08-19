use axum::{
    extract::State,
    http::StatusCode,
    routing::{post},
    Json, Router,
};
use validator::Validate;

use crate::{
    models::{
        AuthResponse, LoginDto, PasswordResetDto, PasswordResetRequestDto,
        RefreshTokenDto, RegisterDto,
    },
    services::auth::{AuthService, Claims},
    state::AppState,
    utils::{AppError, Result},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh_token))
        .route("/logout", post(logout))
        .route("/password-reset/request", post(request_password_reset))
        .route("/password-reset/confirm", post(reset_password))
}

async fn register(
    State(state): State<AppState>,
    Json(dto): Json<RegisterDto>,
) -> Result<(StatusCode, Json<AuthResponse>)> {
    dto.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let response = AuthService::register(&state.db, dto).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn login(
    State(state): State<AppState>,
    Json(dto): Json<LoginDto>,
) -> Result<Json<AuthResponse>> {
    dto.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Check rate limiting
    AuthService::check_rate_limit(&state.db, &dto.email).await?;

    let response = AuthService::login(&state.db, dto).await?;
    Ok(Json(response))
}

async fn refresh_token(
    State(state): State<AppState>,
    Json(dto): Json<RefreshTokenDto>,
) -> Result<Json<AuthResponse>> {
    let response = AuthService::refresh_token(&state.db, dto).await?;
    Ok(Json(response))
}

async fn logout(
    State(state): State<AppState>,
    claims: Claims, // This will come from the auth middleware
) -> Result<StatusCode> {
    AuthService::logout(&state.db, claims.sub).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn request_password_reset(
    State(state): State<AppState>,
    Json(dto): Json<PasswordResetRequestDto>,
) -> Result<StatusCode> {
    dto.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    AuthService::request_password_reset(&state.db, dto).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn reset_password(
    State(state): State<AppState>,
    Json(dto): Json<PasswordResetDto>,
) -> Result<StatusCode> {
    dto.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    AuthService::reset_password(&state.db, dto).await?;
    Ok(StatusCode::NO_CONTENT)
}
