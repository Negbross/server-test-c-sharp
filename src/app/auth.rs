use crate::app::state::AppState;
use crate::config::config::Config;
use crate::core::error::AppError;

use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::RequestPartsExt;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::typed_header::TypedHeader;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Claims {
    pub sub: Uuid,
    pub username: String,
    pub exp: i64,
    pub roles: Vec<String>,
}

pub fn create_token(
    user_id: Uuid,
    username: &str,
    roles: Vec<String>,
    config: &Config,
) -> Result<String, AppError> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claim = Claims {
        sub: user_id,
        username: username.to_owned(),
        exp: expiration,
        roles,
    };

    let jwt_secret = config.jwt_secret.as_bytes();

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(jwt_secret),
    )
    .map_err(|_| AppError::InternalError(String::from("JWT encoding error!")))
}

pub fn decode_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    let secret_key = DecodingKey::from_secret(secret.as_bytes());
    decode::<Claims>(&token, &secret_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|_| AppError::InternalError(String::from("JWT decode error!")))
}

pub async fn get_role(db: &DatabaseConnection, user_uuid: Uuid) -> Result<Vec<String>, AppError> {
    let user_with_role = entity::users::Entity::find()
        .filter(entity::users::Column::Id.eq(user_uuid))
        .find_with_related(entity::generated::prelude::Roles)
        .all(db)
        .await
        .map_err(|_| AppError::NotFound)?
        .into_iter()
        .next();

    if let Some((_user, roles)) = user_with_role {
        Ok(roles.into_iter().map(|r| r.name).collect())
    } else {
        Err(AppError::InvalidToken)
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthUser {
    pub id: Uuid,
    pub username: String,
    pub roles: Vec<String>,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::InternalError(String::from("TypedHeader error!")))?;
        let claims = decode_token(bearer.token(), &app_state.env.jwt_secret)?;
        let roles = get_role(&app_state.database_connection, claims.sub).await?;
        Ok(AuthUser {
            id: claims.sub,
            username: claims.username,
            roles,
        })
    }
}

pub fn require_role(user: &AuthUser, allowed: &[&str]) -> Result<(), AppError> {
    let ok = allowed
        .iter()
        .any(|name| user.roles.iter().any(|r| r == name));
    if !ok {
        return Err(AppError::Forbidden);
    }
    Ok(())
}