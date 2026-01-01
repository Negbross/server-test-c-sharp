use crate::app::controllers::admin_controller;
use crate::app::controllers::product_controller;
use crate::app::controllers::user_controller;
use crate::app::models::product;
use crate::app::models::role;
use crate::app::models::user;
use crate::app::state::AppState;

use axum::extract::DefaultBodyLimit;
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

const CONTENT_LENGTH_LIMIT: usize = 50 * 1024 * 1024;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Dummies API",
        description = "Tester API documentation c sharp",
    ),
    paths(
        // User
        user_controller::register_handler,
        user_controller::login_handler,
        user_controller::get_user,

        // Admin - User Management
        admin_controller::list_all_users,
        admin_controller::get_user_by_id,
        admin_controller::update_user,
        admin_controller::delete_user,
        admin_controller::attach_role_to_user,

        // Admin - Product Management
        admin_controller::list_all_products,

        // Admin - Role Management
        admin_controller::list_all_roles,
        admin_controller::create_role,
        admin_controller::delete_role,

        // Product
        product_controller::create_product,
        product_controller::update_product,
        product_controller::delete_product,
        product_controller::get_all_products,
        product_controller::get_product_by_slug,
    ),
    components(schemas(
        user::User,
        user::RegisterUserPayload,
        user::LoginUserPayload,
        user::LoginResponse,
        user::UserInfo,
        user::UpdateUserPayload,

        product::Product,
        product::CreateProductPayload,
        product::UpdateProductPayload,

        role::Role,
        role::RoleCreatePayload,
    )),
    tags(
        (name = "Auth", description = "Authentication endpoints"),
        (name = "Admin", description = "Admin management endpoints"),
        (name = "Products", description = "Product management endpoints"),
    )
)]
pub struct ApiDocs;

pub fn routes(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let api_docs = ApiDocs::openapi();

    let auth_routes = Router::new()
        .route("/login", post(user_controller::login_handler))
        .route("/register", post(user_controller::register_handler));

    let product_routes = Router::new()
        .route(
            "/",
            get(product_controller::get_all_products).post(product_controller::create_product),
        )
        .route(
            "/{slug}",
            get(product_controller::get_product_by_slug)
                .put(product_controller::update_product)
                .delete(product_controller::delete_product),
        );

    let user_profile_routes = Router::new().route("/", get(user_controller::get_user));

    // Admin routes
    let admin_user_routes = Router::new()
        .route("/", get(admin_controller::list_all_users))
        .route(
            "/{id}",
            get(admin_controller::get_user_by_id)
                .put(admin_controller::update_user)
                .delete(admin_controller::delete_user),
        )
        .route(
            "/{user_id}/roles/{role_id}",
            post(admin_controller::attach_role_to_user),
        );

    let admin_role_routes = Router::new()
        .route(
            "/",
            get(admin_controller::list_all_roles).post(admin_controller::create_role),
        )
        .route("/{id}", delete(admin_controller::delete_role));

    let admin_product_routes = Router::new().route("/", get(admin_controller::list_all_products));

    let admin_routes = Router::new()
        .nest("/users", admin_user_routes)
        .nest("/roles", admin_role_routes)
        .nest("/products", admin_product_routes);

    Router::new()
        .route("/", get(|| async { "hello world" }))
        .nest("/auth", auth_routes)
        .nest("/products", product_routes)
        .nest("/user", user_profile_routes)
        .nest("/admin", admin_routes)
        .merge(SwaggerUi::new("/api-docs").url("/api-docs/openapi.json", api_docs.clone()))
        // Layer
        .layer(TraceLayer::new_for_http())
        .layer(DefaultBodyLimit::max(CONTENT_LENGTH_LIMIT))
        .layer(cors)
        .with_state(state)
}

pub fn handle_error() -> Router {
    Router::new().fallback(get(not_found_error()))
}

/* Map any error into a `500 Internal Server Error` */
pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: ToString,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

// Map str into a `404 Internal Server Error`
pub fn not_found_error() -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, "Not found".to_string())
}
