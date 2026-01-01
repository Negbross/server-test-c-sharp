use crate::app::models::product::{CreateProductPayload, Product, UpdateProductPayload};
use crate::core::error::AppError;
use crate::utils::generate_unique_slug;
use async_trait::async_trait;
use entity::generated::prelude::Products;
use entity::products;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn create(&self, payload: CreateProductPayload) -> Result<Product, AppError>;
    async fn list(&self) -> Result<Vec<Product>, AppError>;
    async fn find_by_slug(&self, slug: &str) -> Result<Option<Product>, AppError>;
    async fn find_by_id(&self, product_id: Uuid) -> Result<Option<Product>, AppError>;
    async fn update(&self, slug: &str, payload: UpdateProductPayload) -> Result<Product, AppError>;
    async fn delete(&self, slug: &str) -> Result<(), AppError>;
}

pub struct SeaormProductRepository {
    db: Arc<DatabaseConnection>,
}

impl SeaormProductRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ProductRepository for SeaormProductRepository {
    async fn create(&self, payload: CreateProductPayload) -> Result<Product, AppError> {
        let new_product = products::ActiveModel {
            sku: Set(payload.sku.to_string()),
            name: Set(payload.name.to_string()),
            slug: Set(generate_unique_slug(payload.name.as_str())),
            description: Set(payload.description.to_string()),
            price: Set(payload.price.to_owned()),
            quantity: Set(payload.quantity.to_owned()),
            ..Default::default()
        };

        let result = new_product.insert(self.db.as_ref()).await?;
        Ok(result.into())
    }

    async fn list(&self) -> Result<Vec<Product>, AppError> {
        let models = Products::find().all(self.db.as_ref()).await?;
        let products = models.into_iter().map(|m| m.into()).collect();
        Ok(products)
    }

    async fn find_by_slug(&self, slug: &str) -> Result<Option<Product>, AppError> {
        let model = products::Entity::find()
            .filter(products::Column::Slug.eq(slug))
            .one(self.db.as_ref())
            .await?;
        Ok(model.map(|m| m.into()))
    }

    async fn find_by_id(&self, product_id: Uuid) -> Result<Option<Product>, AppError> {
        let model = products::Entity::find()
            .filter(products::Column::Id.eq(product_id))
            .one(self.db.as_ref())
            .await?;
        Ok(model.map(|m| m.into()))
    }

    async fn update(&self, slug: &str, payload: UpdateProductPayload) -> Result<Product, AppError> {
        let model = Products::find()
            .filter(products::Column::Slug.eq(slug))
            .one(self.db.as_ref())
            .await?
            .ok_or(AppError::NotFound)?;
        let mut active_model = model.into_active_model();
        active_model.name = Set(payload.name.to_string());
        active_model.description = Set(payload.description.to_string());
        active_model.price = Set(payload.price);
        active_model.quantity = Set(payload.quantity);
        let result = active_model.update(self.db.as_ref()).await?;

        Ok(result.into())
    }

    async fn delete(&self, slug: &str) -> Result<(), AppError> {
        let model = Products::delete_many()
            .filter(products::Column::Slug.eq(slug))
            .exec(self.db.as_ref())
            .await?;

        if model.rows_affected == 0 {
            return Err(AppError::NotFound);
        }

        Ok(())
    }
}
