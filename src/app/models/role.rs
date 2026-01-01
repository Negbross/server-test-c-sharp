use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Role {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: uuid::Uuid,
    #[schema(example = "Administrator")]
    pub name: String,
}

impl From<entity::roles::Model> for Role {
    fn from(model: entity::roles::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RoleCreatePayload {
    #[schema(example = "Editor")]
    pub name: String,
}
