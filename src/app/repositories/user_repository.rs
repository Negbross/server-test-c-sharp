use crate::app::models::user::{RegisterUserPayload, UpdateUserPayload, User, UserInfo};
use crate::core::error::AppError;
use async_trait::async_trait;
use entity::generated::prelude::Users;
use entity::users;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
    RuntimeErr, Set,
};
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, payload: RegisterUserPayload) -> Result<User, AppError>;
    async fn list_all(&self) -> Result<Vec<User>, AppError>;
    async fn find_by_unique_identifier(&self, identifier: &str) -> Result<Option<User>, AppError>;
    async fn find_by_username_or_email(&self, identifier: &str) -> Result<Option<User>, AppError>;
    async fn update(&self, username: &str, payload: UpdateUserPayload) -> Result<User, AppError>;
    async fn delete(&self, username: &str) -> Result<(), AppError>;
    async fn attach_role(&self, user_id: Uuid, role_id: Uuid) -> Result<(), AppError>;
}

pub struct SeaormUserRepository {
    db: Arc<DatabaseConnection>,
}

impl SeaormUserRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    fn identifier_condition(identifier: &str) -> Condition {
        let mut cond = Condition::any()
            .add(users::Column::Username.eq(identifier))
            .add(users::Column::Email.eq(identifier));

        if let Ok(id) = Uuid::parse_str(identifier) {
            cond = cond.add(users::Column::Id.eq(id));
        }

        cond
    }

    async fn find_model_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<Option<users::Model>, AppError> {
        let user = users::Entity::find()
            .filter(Condition::all().add(Self::identifier_condition(identifier)))
            .one(self.db.as_ref())
            .await?;
        Ok(user)
    }

    async fn find_user_with_roles(&self, identifier: &str) -> Result<Option<User>, AppError> {
        let user_with_roles = users::Entity::find()
            .filter(Condition::all().add(Self::identifier_condition(identifier)))
            .find_with_related(entity::generated::prelude::Roles)
            .all(self.db.as_ref())
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?
            .into_iter()
            .next();

        Ok(user_with_roles
            .into_iter()
            .next()
            .map(|(user_model, role_models)| (user_model, role_models).into()))
    }

    async fn find_all_with_roles(&self) -> Result<Vec<User>, AppError> {
        let users_with_roles = users::Entity::find()
            .find_with_related(entity::generated::prelude::Roles)
            .all(self.db.as_ref())
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        Ok(users_with_roles
            .into_iter()
            .map(|(user_model, role_models)| (user_model, role_models).into())
            .collect())
    }
}

#[async_trait]
impl UserRepository for SeaormUserRepository {
    async fn create(&self, payload: RegisterUserPayload) -> Result<User, AppError> {
        let new_user = users::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(payload.name),
            username: Set(payload.username),
            email: Set(payload.email),
            password: Set(payload.password),
            ..Default::default()
        };
        match new_user.insert(self.db.as_ref()).await {
            Ok(model) => {
                self.find_user_with_roles(&model.username)
                    .await?
                    .ok_or(AppError::InternalError("User already exists".to_string()))
            },
            Err(db_err) => {
                // Periksa apakah ini error dari database
                if let DbErr::Query(RuntimeErr::SqlxError(sqlx_error)) = db_err {
                    // Periksa apakah error ini adalah error dari database
                    if let Some(db_err_info) = sqlx_error.as_database_error() {
                        // Dapatkan kode error spesifik (contoh: "23505" untuk unique violation di Postgres)
                        if db_err_info.code().as_deref() == Some("23505") {
                            return Err(AppError::DuplicateEntry(
                                "Email atau username sudah terdaftar.".to_string(),
                            ));
                        }
                    }
                }
                // Jika bukan error unique, kembalikan sebagai error database biasa
                Err(AppError::InternalError(
                    "Gagal membuat pengguna.".to_string(),
                ))
            }
        }
    }

    async fn list_all(&self) -> Result<Vec<User>, AppError> {
        self.find_all_with_roles().await
    }

    async fn find_by_unique_identifier(&self, identifier: &str) -> Result<Option<User>, AppError> {
        let is_uuid = Uuid::parse_str(identifier).is_ok();

        let model = Users::find()
            .filter(
                Condition::any()
                    .add(if is_uuid {
                        users::Column::Id.eq(identifier)
                    } else {
                        // Kalo bukan uuid, maka gak cocok
                        users::Column::Id.eq(Uuid::nil())
                    })
                    .add(Self::identifier_condition(identifier)),
            )
            .one(self.db.as_ref())
            .await?
            .ok_or(AppError::NotFound)?;
        let user_with_roles = self.find_user_with_roles(&model.username).await?
            .ok_or(AppError::NotFound)?;
        Ok(Some(user_with_roles.into()))
    }

    async fn find_by_username_or_email(&self, identifier: &str) -> Result<Option<User>, AppError> {
        let user_with_roles = self.find_user_with_roles(identifier).await?;

        if let Some(user_with_roles) = user_with_roles {
            Ok(Some(user_with_roles.into()))
        } else if let None = user_with_roles {
            Ok(None)
        } else {
            Err(AppError::NotFound)
        }
    }

    async fn update(&self, username: &str, payload: UpdateUserPayload) -> Result<User, AppError> {
        let model = self.find_model_by_identifier(username).await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        match model {
            Some(model) => {
                let user_id = model.id;
                let mut active_model: users::ActiveModel = model.into();
                if let Some(name) = payload.name {
                    active_model.name = Set(name.to_string());
                }
                if let Some(username) = payload.username {
                    active_model.username = Set(username.to_string());
                }
                if let Some(email) = payload.email {
                    active_model.email = Set(email.to_string());
                }
                if let Some(password) = payload.password {
                    active_model.password = Set(password.to_string());
                }
                active_model
                    .update(self.db.as_ref())
                    .await
                    .map_err(|e| AppError::InternalError(e.to_string()))?;

                // Fetch updated user with roles
                self.find_user_with_roles(&user_id.to_string())
                    .await?
                    .ok_or(AppError::NotFound)
            }
            None => Err(AppError::NotFound),
        }
    }

    async fn delete(&self, username: &str) -> Result<(), AppError> {
        let existing_user = users::Entity::delete_many()
            .filter(users::Column::Username.eq(username))
            .exec(self.db.as_ref())
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;
        if existing_user.rows_affected == 0 {
            return Err(AppError::NotFound);
        }
        Ok(())
    }

    async fn attach_role(&self, user_id: Uuid, role_id: Uuid) -> Result<(), AppError> {
        let existing = users::Entity::find()
            .filter(users::Column::Id.eq(user_id))
            .one(self.db.as_ref())
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        if existing.is_none() {
            return Err(AppError::NotFound);
        }

        let new_role = entity::user_roles::ActiveModel {
            user_id: Set(existing.unwrap().id),
            role_id: Set(role_id),
            ..Default::default()
        };
        new_role.insert(self.db.as_ref()).await?;
        Ok(())
    }
}
