use crate::app::repositories::product_repository::SeaormProductRepository;
use crate::app::repositories::role_repository::SeaORMRoleRepository;
use crate::app::repositories::user_repository::SeaormUserRepository;
use crate::app::services::admin_service::AdminService;
use crate::app::services::product_service::ProductService;
use crate::app::services::user_service::UserService;
use crate::config::config::Config;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub database_connection: DatabaseConnection,
    pub env: Config,
    pub product_service: Arc<ProductService>,
    pub user_service: Arc<UserService>,
    pub admin_service: Arc<AdminService>,
}

impl AppState {
    pub async fn new() -> Self {
        let config = Arc::new(Config::init()).as_ref().clone();
        let db_connection = crate::config::database::connect(&config)
            .await
            .expect("Failed to connect to the database");

        let db_arc = Arc::from(db_connection.clone());

        let product_repo = Arc::new(SeaormProductRepository::new(db_arc.clone()));
        let user_repo = Arc::new(SeaormUserRepository::new(db_arc.clone()));
        let role_repo = Arc::new(SeaORMRoleRepository::new(db_arc.clone()));

        let product_service = Arc::new(ProductService::new(product_repo.clone()));
        let user_service = Arc::new(UserService::new(user_repo.clone()));
        let admin_service = Arc::new(AdminService::new(user_repo, product_repo, role_repo));

        Self {
            database_connection: db_connection,
            env: config,
            product_service,
            user_service,
            admin_service,
        }
    }
}
