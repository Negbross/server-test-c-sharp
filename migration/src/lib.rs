pub use sea_orm_migration::prelude::*;

pub struct Migrator;

pub mod m20250720_082026_products;
pub mod m20250723_074116_users;
mod m20250725_111229_roles;
mod m20250725_112514_user_roles;
mod m20251230_011212_permissions;
mod m20251230_011219_role_has_permissions;


#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250723_074116_users::Migration),
            Box::new(m20250720_082026_products::Migration),
            Box::new(m20250725_111229_roles::Migration),
            Box::new(m20250725_112514_user_roles::Migration),
            Box::new(m20251230_011212_permissions::Migration),
            Box::new(m20251230_011219_role_has_permissions::Migration),
        ]
    }
}

