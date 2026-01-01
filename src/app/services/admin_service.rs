use crate::app::models::product::Product;
use crate::app::models::role::{Role, RoleCreatePayload};
use crate::app::models::user::{UpdateUserPayload, User};
use crate::app::repositories::product_repository::ProductRepository;
use crate::app::repositories::role_repository::RoleRepository;
use crate::app::repositories::user_repository::UserRepository;
use crate::core::error::AppError;
use std::sync::Arc;
use uuid::Uuid;

pub struct AdminService {
    user_repo: Arc<dyn UserRepository>,
    product_repo: Arc<dyn ProductRepository>,
    role_repo: Arc<dyn RoleRepository>,
}

impl AdminService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        product_repo: Arc<dyn ProductRepository>,
        role_repo: Arc<dyn RoleRepository>,
    ) -> Self {
        Self {
            user_repo,
            product_repo,
            role_repo,
        }
    }

    pub async fn list_all_users(&self) -> Result<Vec<User>, AppError> {
        self.user_repo.list_all().await
    }

    pub async fn list_all_products(&self) -> Result<Vec<Product>, AppError> {
        self.product_repo.list().await
    }

    pub async fn list_all_roles(&self) -> Result<Vec<Role>, AppError> {
        self.role_repo.list().await
    }

    pub async fn get_user_by_id(&self, user_id: &str) -> Result<Option<User>, AppError> {
        self.user_repo.find_by_unique_identifier(user_id).await
    }

    pub async fn get_user_by_username_or_email(
        &self,
        identifier: &str,
    ) -> Result<Option<User>, AppError> {
        self.user_repo.find_by_username_or_email(identifier).await
    }

    pub async fn update_user(
        &self,
        user_id: &str,
        payload: UpdateUserPayload,
    ) -> Result<User, AppError> {
        self.user_repo.update(user_id, payload).await
    }

    pub async fn delete_user_by_name(&self, user_id: &str) -> Result<(), AppError> {
        self.user_repo.delete(user_id).await
    }

    pub async fn attach_role_to_user(&self, user_id: Uuid, role_id: Uuid) -> Result<(), AppError> {
        // Verify role exists
        self.role_repo
            .find_by_id(role_id)
            .await?
            .ok_or(AppError::NotFound)?;

        self.user_repo.attach_role(user_id, role_id).await
    }

    pub async fn create_role(&self, payload: RoleCreatePayload) -> Result<Role, AppError> {
        // Check if role with same name exists
        if let Ok(Some(_)) = self.role_repo.find_exact_name(&payload.name).await {
            return Err(AppError::DuplicateEntry(
                "Role with this name already exists".to_string(),
            ));
        }
        self.role_repo.create(payload).await
    }

    pub async fn delete_role(&self, role_id: Uuid) -> Result<(), AppError> {
        self.role_repo.delete(role_id).await
    }
}
