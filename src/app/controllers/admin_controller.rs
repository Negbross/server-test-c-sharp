use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use uuid::Uuid;

use crate::app::models::product::Product;
use crate::app::models::role::{Role, RoleCreatePayload};
use crate::app::models::user::{UpdateUserPayload, User};
use crate::app::state::AppState;
use crate::core::error::AppError;

// ============== USER MANAGEMENT ==============

#[utoipa::path(
    get,
    path = "/admin/users",
    responses(
        (status = 200, description = "List all users", body = Vec<User>),
        (status = 500, description = "Internal server error")
    ),
    tag = "Admin"
)]
pub async fn list_all_users(State(state): State<AppState>) -> impl IntoResponse {
    match state.admin_service.list_all_users().await {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(e) => e.into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/admin/users/{id}",
    params(
        ("id" = String, Path, description = "User ID, username, or email")
    ),
    responses(
        (status = 200, description = "Get user by ID", body = User),
        (status = 404, description = "User not found")
    ),
    tag = "Admin"
)]
pub async fn get_user_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.admin_service.get_user_by_id(&id).await {
        Ok(Some(user)) => (StatusCode::OK, Json(user)).into_response(),
        Ok(None) => AppError::NotFound.into_response(),
        Err(e) => e.into_response(),
    }
}

#[utoipa::path(
    put,
    path = "/admin/users/{id}",
    params(
        ("id" = String, Path, description = "User ID, username, or email")
    ),
    request_body = UpdateUserPayload,
    responses(
        (status = 200, description = "Update user", body = User),
        (status = 404, description = "User not found")
    ),
    tag = "Admin"
)]
pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateUserPayload>,
) -> impl IntoResponse {
    match state.admin_service.update_user(&id, payload).await {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(e) => e.into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/admin/users/{id}",
    params(
        ("id" = String, Path, description = "Username to delete")
    ),
    responses(
        (status = 200, description = "User deleted"),
        (status = 404, description = "User not found")
    ),
    tag = "Admin"
)]
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.admin_service.delete_user_by_name(&id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({"message": "User deleted"})),
        )
            .into_response(),
        Err(e) => e.into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/admin/users/{user_id}/roles/{role_id}",
    params(
        ("user_id" = Uuid, Path, description = "User UUID"),
        ("role_id" = Uuid, Path, description = "Role UUID")
    ),
    responses(
        (status = 200, description = "Role attached to user"),
        (status = 404, description = "User or role not found")
    ),
    tag = "Admin"
)]
pub async fn attach_role_to_user(
    State(state): State<AppState>,
    Path((user_id, role_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    match state
        .admin_service
        .attach_role_to_user(user_id, role_id)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({"message": "Role attached to user"})),
        )
            .into_response(),
        Err(e) => e.into_response(),
    }
}

// ============== PRODUCT MANAGEMENT ==============

#[utoipa::path(
    get,
    path = "/admin/products",
    responses(
        (status = 200, description = "List all products", body = Vec<Product>),
        (status = 500, description = "Internal server error")
    ),
    tag = "Admin"
)]
pub async fn list_all_products(State(state): State<AppState>) -> impl IntoResponse {
    match state.admin_service.list_all_products().await {
        Ok(products) => (StatusCode::OK, Json(products)).into_response(),
        Err(e) => e.into_response(),
    }
}

// ============== ROLE MANAGEMENT ==============

#[utoipa::path(
    get,
    path = "/admin/roles",
    responses(
        (status = 200, description = "List all roles", body = Vec<Role>),
        (status = 500, description = "Internal server error")
    ),
    tag = "Admin"
)]
pub async fn list_all_roles(State(state): State<AppState>) -> impl IntoResponse {
    match state.admin_service.list_all_roles().await {
        Ok(roles) => (StatusCode::OK, Json(roles)).into_response(),
        Err(e) => e.into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/admin/roles",
    request_body = RoleCreatePayload,
    responses(
        (status = 201, description = "Role created", body = Role),
        (status = 409, description = "Role already exists")
    ),
    tag = "Admin"
)]
pub async fn create_role(
    State(state): State<AppState>,
    Json(payload): Json<RoleCreatePayload>,
) -> impl IntoResponse {
    match state.admin_service.create_role(payload).await {
        Ok(role) => (StatusCode::CREATED, Json(role)).into_response(),
        Err(e) => e.into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/admin/roles/{id}",
    params(
        ("id" = Uuid, Path, description = "Role UUID")
    ),
    responses(
        (status = 200, description = "Role deleted"),
        (status = 404, description = "Role not found")
    ),
    tag = "Admin"
)]
pub async fn delete_role(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match state.admin_service.delete_role(id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({"message": "Role deleted"})),
        )
            .into_response(),
        Err(e) => e.into_response(),
    }
}
