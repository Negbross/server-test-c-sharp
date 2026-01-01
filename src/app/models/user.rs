use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct User {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    #[schema(example = "John Doe")]
    pub name: String,
    #[schema(example = "johndoe")]
    pub username: String,
    #[schema(example = "john@example.com")]
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub roles: Vec<String>
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct RegisterUserPayload {
    #[validate(length(
        min = 3,
        max = 10,
        message = "name length should be at least 3 (max 10)"
    ))]
    #[schema(example = "johndoe")]
    pub username: String,
    #[validate(length(min = 3, message = "Nama minimal 3 karakter."))]
    #[schema(example = "John Doe")]
    pub name: String,
    #[validate(length(min = 8, message = "Password minimal 8 karakter."))]
    #[schema(example = "secret123")]
    pub password: String,
    // #[validate(email(message = "Format email tidak valid."))]
    #[schema(example = "john@example.com")]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateUserPayload {
    #[validate(length(
        min = 3,
        max = 10,
        message = "name length should be at least 3 (max 10)"
    ))]
    pub username: Option<String>,
    #[validate(length(min = 3, message = "Nama minimal 3 karakter."))]
    pub name: Option<String>,
    #[validate(length(min = 8, message = "Password minimal 8 karakter."))]
    pub password: Option<String>,
    // #[validate(email(message = "Format email tidak valid."))]
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct LoginUserPayload {
    #[schema(example = "johndoe")]
    pub identifier: String,
    #[schema(example = "secret123")]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UserInfo {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    #[schema(example = "John Doe")]
    pub full_name: String,
    #[schema(example = "johndoe")]
    pub username: String,
    #[schema(example = "john@example.com")]
    pub email: String,
    #[schema(example = json!(["admin", "user"]))]
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub token: String,
    pub user: UserInfo,
}

impl From<(entity::users::Model, Vec<entity::roles::Model>)> for User {
    fn from((user_model, role_model): (entity::users::Model, Vec<entity::roles::Model>)) -> Self {
        Self {
            id: user_model.id,
            name: user_model.name,
            username: user_model.username,
            email: user_model.email,
            password: user_model.password,
            roles: role_model.into_iter().map(|r| r.name).collect(),
        }
    }
}
