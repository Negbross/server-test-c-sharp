use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, ColumnTrait};
use uuid::Uuid;
use entity::roles;
use crate::app::models::role::{Role, RoleCreatePayload};
use crate::core::error::AppError;

#[async_trait]
pub trait RoleRepository: Send + Sync {
    async fn create(&self, payload: RoleCreatePayload) -> Result<Role, AppError>;
    async fn find_exact_name(&self, role_name: &str) -> Result<Option<Role>, AppError>;
    async fn find_by_id(&self, role_id: Uuid) -> Result<Option<Role>, AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
    async fn list(&self) -> Result<Vec<Role>, AppError>;
}

pub struct SeaORMRoleRepository {
    db: Arc<DatabaseConnection>
}

impl SeaORMRoleRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> SeaORMRoleRepository {
        SeaORMRoleRepository { db }
    }
}

#[async_trait]
impl RoleRepository for SeaORMRoleRepository {
    async fn create(&self, payload: RoleCreatePayload) -> Result<Role, AppError> {
        let new_role = roles::ActiveModel {
            name: Set(payload.name.to_owned()),
            ..Default::default()
        };
        match new_role.insert(self.db.as_ref()).await {
            Ok(model) => Ok(model.into()),
            Err(db_error) => {
                Err(AppError::from(db_error))
            }
        }
    }

    async fn find_exact_name(&self, role_name: &str) -> Result<Option<Role>, AppError> {
        let exist_role = roles::Entity::find()
            .filter(roles::Column::Name.eq(role_name))
            .one(self.db.as_ref())
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(Some(exist_role.into()))
    }

    async fn find_by_id(&self, role_id: Uuid) -> Result<Option<Role>, AppError> {
        let exist_role = roles::Entity::find()
            .filter(roles::Column::Id.eq(role_id))
            .one(self.db.as_ref())
            .await?
            .ok_or(AppError::NotFound)?;
        Ok(Some(exist_role.into()))
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let role_exist = roles::Entity::delete_many()
            .filter(roles::Column::Id.eq(id))
            .exec(self.db.as_ref())
            .await?;
        if role_exist.rows_affected == 0 {
            return Err(AppError::NotFound)?;
        }
        Ok(())
    }

    async fn list(&self) -> Result<Vec<Role>, AppError> {
        let roles = roles::Entity::find().all(self.db.as_ref()).await
            .map_err(|_| AppError::NotFound)?;
        Ok(roles.into_iter().map(|role| role.into()).collect())
    }
}