use std::collections::HashMap;
use rust_decimal_macros::dec;
use sea_orm::{ColumnTrait, ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait, QueryFilter, Set};
use uuid::Uuid;
use entity::{user_roles, users, roles, permissions, role_has_permissions, products};
use crate::app::hashing::hash::hash_password;
use crate::mock::factory::{PermissionFactory, ProductFactory, RoleFactory, UserFactory};

pub async fn seed_all(db: &DatabaseConnection) -> Result<(), DbErr> {
    println!("Seed Roles");
    seed_roles(db).await?;
    println!("Seed Permissions");
    seed_permissions(db).await?;
    println!("Seed Role has permissions");
    seed_role_has_permission(db).await?;
    println!("Seed admin user");
    let admin_id = seed_admin_user(db).await?;
    println!("Seed products");
    seed_products(db, admin_id).await
}

async fn ensure_role(db: &DatabaseConnection, name: &str) -> Result<roles::Model, DbErr> {
    if let Some(r) = roles::Entity::find()
        .filter(roles::Column::Name.eq(name))
        .one(db)
        .await? {
        return Ok(r);
    };

    roles::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(name.to_owned()),
        ..Default::default()
    }.insert(db).await
}

async fn attach_roles(
    db: &DatabaseConnection,
    user_id: Uuid,
    role_names: &[&str],
) -> Result<(), sea_orm::DbErr> {
    for role_name in role_names {
        let role = ensure_role(db, role_name).await?;

        let exists = user_roles::Entity::find()
            .filter(user_roles::Column::UserId.eq(user_id))
            .filter(user_roles::Column::RoleId.eq(role.id))
            .one(db)
            .await?
            .is_some();

        if !exists {
            user_roles::ActiveModel {
                user_id: Set(user_id),
                role_id: Set(role.id),
                ..Default::default()
            }
                .insert(db)
                .await?;
        }
    }
    Ok(())
}

async fn seed_permissions(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    let permissions = vec!["all", "read", "create", "update", "delete"];

    for name in permissions {
        // cek apakah permission sudah ada
        let exists = permissions::Entity::find()
            .filter(permissions::Column::NamePermission.eq(name))
            .one(db)
            .await?
            .is_some();

        if exists {
            continue;
        }

        let model = PermissionFactory::new().name(name.to_string()).build();

        let _ = model.insert(db).await?;
    }

    Ok(())
}

async fn seed_roles(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    let roles = vec!["admin", "manager", "staff", "sales", "guest"];

    for name in roles {
        // cek apakah role sudah ada
        let exists = roles::Entity::find()
            .filter(roles::Column::Name.eq(name))
            .one(db)
            .await?
            .is_some();

        if exists {
            continue;
        }

        let model = RoleFactory::new().name(name.to_string()).build();

        let _ = model.insert(db).await?;
    }

    Ok(())
}

async fn seed_role_has_permission(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    let role_permission_map = vec![
        (
            "admin",
            vec!["all", "read", "create", "update", "delete"],
        ),
        (
            "staff",
            vec!["read", "create", "update"],
        ),
        (
            "user",
            vec!["read"],
        ),
    ];

    let all_role = roles::Entity::find().all(db).await?;
    let all_permissions = permissions::Entity::find().all(db).await?;

    let role_map: HashMap<String, Uuid> = all_role.into_iter()
        .map(|r| (r.name.clone(), r.id))
        .collect();

    let permission_map: HashMap<String, Uuid> = all_permissions.into_iter()
        .map(|perm| (perm.name_permission.clone(), perm.id))
        .collect();

    for (role_name, perm_names) in role_permission_map {
        let role_id = match role_map.get(role_name) {
            Some(role_id) => *role_id,
            None => {
                ensure_role(db, role_name).await?.id;
                continue
            }
        };

        for perm_name in perm_names {
            let perm_exist = match permission_map.get(perm_name) {
                Some(perm_exist) => *perm_exist,
                None => {
                    println!("Gada permission {}", perm_name);
                    continue
                }
            };

            let exists = role_has_permissions::Entity::find()
                .filter(role_has_permissions::Column::RoleId.eq(role_id))
                .filter(role_has_permissions::Column::PermissionId.eq(perm_exist))
                .one(db)
                .await?;
            if exists.is_none() {
                role_has_permissions::ActiveModel {
                    id: Set(Uuid::new_v4()),
                    role_id: Set(role_id),
                    permission_id: Set(perm_exist),
                }.insert(db)
                    .await?;
                println!("Assigned '{}' to '{}'", perm_name, role_name)
            }
        }
        
    }

    Ok(())
}

async fn seed_admin_user(db: &DatabaseConnection) -> Result<Uuid, sea_orm::DbErr> {
    if let Some(user) = users::Entity::find().one(db).await? {
        return Ok(user.id);
    }

    let password_hash = hash_password("12345678")
        .expect("Error hashing password");
    let admin = UserFactory::new()
        .id(Uuid::new_v4())
        .username("superadmin".to_string())
        .name("Super Admin".to_string())
        .email("superadmin@example.com".to_string())
        .password(password_hash)
        .build()
        .insert(db)
        .await?;
    Ok(admin.id)
}

async fn seed_products(db: &DatabaseConnection, admin_id: Uuid) -> Result<(), sea_orm::DbErr> {
    // kalau sudah ada product, skip
    let count = products::Entity::find().count(db).await?;
    if count > 0 {
        return Ok(());
    }

    ProductFactory::new(admin_id)
        .name("Keyboard Mechanical V1".to_string())
        .slug("keyboard-mechanical-v1".to_string())
        .sku("KB-MECH-001".to_string())
        .price(dec!(99.99))
        .quantity(20)
        .build()
        .insert(db)
        .await?;

    ProductFactory::new(admin_id)
        .name("Wireless Mouse Pro".to_string())
        .slug("wireless-mouse-pro".to_string())
        .sku("MS-WLS-001".to_string())
        .price(dec!(50.00))
        .quantity(50)
        .build()
        .insert(db)
        .await?;

    Ok(())
}