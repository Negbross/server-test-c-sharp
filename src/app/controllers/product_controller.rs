use crate::app::models::product::{CreateProductPayload, UpdateProductPayload, Product};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use crate::app::state::AppState;

#[utoipa::path(
    post,
    path = "/products",
    responses(
        (status = 200, description = "Create a product", body = Product)
    )
)]
pub async fn create_product(
    State(state): State<AppState>,
    Json(json): Json<CreateProductPayload>,
) -> impl IntoResponse {
    match state.product_service.create_product(json).await {
        Ok(product) => (StatusCode::CREATED, Json(product)).into_response(),
        Err(e) => e.into_response()
    }
}

#[utoipa::path(
    put,
    path = "/products/{slug}",
    request_body = UpdateProductPayload,
    responses(
        (status = 200, description = "Update a product", body = Product),
    )
)]
pub async fn update_product(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(json): Json<UpdateProductPayload>,
) -> impl IntoResponse {
    match state.product_service.update_product(&slug, json).await {
        Ok(product) => (StatusCode::OK, Json(product)).into_response(),
        Err(e) => e.into_response()
    }
}

#[utoipa::path(
    delete,
    path = "/products/{slug}",
    responses(
        (status = 200, description = "Delete a product"),
    )
)]
pub async fn delete_product(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    match state.product_service.delete_product(&slug).await {
        Ok(product) => (StatusCode::OK, Json(product)).into_response(),
        Err(e) => e.into_response()
    }
}

#[utoipa::path(
    post,
    path = "/products",
    responses(
        (status = 200, description = "List all products", body = Vec<Product>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_all_products(State(state): State<AppState>) -> impl IntoResponse {
    match state.product_service.get_all_products().await {
        Ok(products) => (StatusCode::OK, Json(products)).into_response(),
        Err(e) => e.into_response()
    }
}

#[utoipa::path(
    post,
    path = "/products/{slug}",
    responses(
        (status = 200, description = "Get current user", body = Product),
    )
)]
pub async fn get_product_by_slug(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    match state.product_service.get_product_by_slug(&slug).await {
        Ok(product) => (StatusCode::OK, Json(product)).into_response(),
        Err(e) => e.into_response()
    }
}