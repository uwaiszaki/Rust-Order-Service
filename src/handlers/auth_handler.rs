use axum::{
    async_trait,
    extract::{FromRequest, RequestParts, Extension},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use crate::models::auth::{LoginRequest, Claims};
use crate::services::auth_service;
use crate::state::AppState;
use crate::app_config::get_config;
use std::sync::Arc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use validator::Validate;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct AuthUser {
    pub user_id: i32,
    pub role: String,
}

#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
    MissingToken,
    WrongCredentials,
    ValidationError,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing token"),
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::ValidationError => (StatusCode::BAD_REQUEST, "Validation error"),
        };
        (status, message).into_response()
    }
}

pub async fn login(Json(login): Json<LoginRequest>) -> Result<impl IntoResponse, AuthError> {
    if let Err(_) = login.validate() {
        return Err(AuthError::ValidationError);
    }

    let user = auth_service::login(login)
        .await
        .map_err(|_| AuthError::WrongCredentials)?;

    let config = get_config();
    let key = EncodingKey::from_secret(config.jwt_secret.as_bytes());
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    let claims = Claims {
        sub: user.id,
        role: user.role,
        exp: (now + 3600) as usize,
    };

    let token = encode(&Header::default(), &claims, &key)
        .map_err(|_| AuthError::InvalidToken)?;

    Ok(Json(serde_json::json!({
        "token": token,
        "token_type": "Bearer"
    })))
}

#[async_trait]
impl<B> FromRequest<B> for AuthUser
where
    B: Send,
{
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let headers = req.headers().ok_or(AuthError::MissingToken)?;
        let auth_header = headers
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .ok_or(AuthError::MissingToken)?;

        if !auth_header.starts_with("Bearer ") {
            return Err(AuthError::InvalidToken);
        }

        let token = auth_header.trim_start_matches("Bearer ");
        let config = get_config();
        let key = DecodingKey::from_secret(config.jwt_secret.as_bytes());
        let validation = Validation::new(Algorithm::HS256);

        let token_data = decode::<Claims>(token, &key, &validation)
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(AuthUser {
            user_id: token_data.claims.sub,
            role: token_data.claims.role,
        })
    }
}

pub async fn admin_only(_: AdminUser) -> Result<impl IntoResponse, AuthError> {
    Ok(Json(serde_json::json!({
        "message": "Welcome to the admin area!"
    })))
}

#[derive(Debug)]
pub struct AdminUser(pub AuthUser);

#[async_trait]
impl<B> FromRequest<B> for AdminUser
where
    B: Send,
{
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let auth_user = AuthUser::from_request(req).await?;
        
        if auth_user.role != "admin" {
            return Err(AuthError::InvalidToken);
        }

        Ok(AdminUser(auth_user))
    }
} 