use crate::app::auth::AuthUser;
use crate::app::models::user::{LoginResponse, LoginUserPayload, UserInfo, RegisterUserPayload};
use crate::app::state::AppState;
use crate::core::error::AppError;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = RegisterUserPayload,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 409, description = "User already exist")
    )
)]
pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUserPayload>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        return AppError::Validation(e.to_string()).into_response();
    }

    match state.user_service.register_handler(payload).await {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(e) => e.into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginUserPayload,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials")
    )
)]
pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginUserPayload>,
) -> impl IntoResponse {
    let config = state.env.clone();

    match state.user_service.login_handler(payload, &config).await {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(e) => e.into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/user",
    responses(
        (status = 200, description = "Get current user", body = UserInfo),
    )
)]
pub async fn get_user(auth_user: AuthUser) -> impl IntoResponse {
    (StatusCode::OK, Json(auth_user)).into_response()
}
