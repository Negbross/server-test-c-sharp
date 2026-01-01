use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Product {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    #[schema(example = "PROD-001")]
    pub sku: String,
    #[schema(example = "Mechanical Keyboard")]
    pub name: String,
    #[schema(example = "mechanical-keyboard")]
    pub slug: String,
    #[schema(example = "A high-quality mechanical keyboard with RGB lighting.")]
    pub description: String,
    #[schema(example = "1500.00")]
    pub price: Decimal,
    #[schema(example = 50)]
    pub quantity: i64,
}

impl From<entity::products::Model> for Product {
    fn from(model: entity::products::Model) -> Self {
        Self {
            id: model.id,
            sku: model.sku,
            name: model.name,
            slug: model.slug,
            description: model.description,
            price: model.price,
            quantity: model.quantity,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateProductPayload {
    #[schema(example = "PROD-001")]
    pub sku: String,
    #[schema(example = "Mechanical Keyboard")]
    pub name: String,
    #[schema(example = "mechanical-keyboard-uwtrw4")]
    pub slug: String,
    #[schema(example = "A high-quality mechanical keyboard with RGB lighting.")]
    pub description: String,
    #[schema(example = "1500.00")]
    pub price: Decimal,
    #[schema(example = 50)]
    pub quantity: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateProductPayload {
    #[schema(example = "PROD-001")]
    pub sku: String,
    #[schema(example = "Mechanical Keyboard")]
    pub name: String,
    #[schema(example = "mechanical-keyboard")]
    pub slug: String,
    #[schema(example = "A high-quality mechanical keyboard with RGB lighting.")]
    pub description: String,
    #[schema(example = "1500.00")]
    pub price: Decimal,
    #[schema(example = 50)]
    pub quantity: i64,
}
