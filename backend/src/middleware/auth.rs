use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{header, request::Parts, StatusCode},
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use uuid::Uuid;

use crate::{
    config::Config,
    services::auth::{AuthService, Claims},
    state::AppState,
    utils::AppError,
};

/// Extractor for JWT claims that validates the token
#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::Unauthorized)?;

        // Get the app state to access config
        let app_state = AppState::from_ref(state);
        
        // Validate the JWT token
        let claims = AuthService::validate_jwt(bearer.token(), &app_state.config)?;

        Ok(claims)
    }
}

/// Optional claims extractor that doesn't fail if no token is present
pub struct OptionalClaims(pub Option<Claims>);

#[async_trait]
impl<S> FromRequestParts<S> for OptionalClaims
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Try to extract authorization header
        let auth_header = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .ok();

        if let Some(TypedHeader(Authorization(bearer))) = auth_header {
            // Get the app state to access config
            let app_state = AppState::from_ref(state);
            
            // Try to validate the JWT token
            match AuthService::validate_jwt(bearer.token(), &app_state.config) {
                Ok(claims) => Ok(OptionalClaims(Some(claims))),
                Err(_) => Ok(OptionalClaims(None)),
            }
        } else {
            Ok(OptionalClaims(None))
        }
    }
}

/// Simple user ID extractor that gets the user ID from JWT claims
pub struct UserId(pub Uuid);

#[async_trait]
impl<S> FromRequestParts<S> for UserId
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let claims = Claims::from_request_parts(parts, state).await?;
        Ok(UserId(claims.sub))
    }
}

/// Optional user ID extractor
pub struct OptionalUserId(pub Option<Uuid>);

#[async_trait]
impl<S> FromRequestParts<S> for OptionalUserId
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let optional_claims = OptionalClaims::from_request_parts(parts, state).await?;
        Ok(OptionalUserId(optional_claims.0.map(|c| c.sub)))
    }
}
