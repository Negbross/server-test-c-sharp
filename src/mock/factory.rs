use entity::{permissions, products, roles, users};
use rust_decimal::Decimal;
use sea_orm::Set;
use uuid::Uuid;

pub struct RoleFactory {
    name: String,
}

impl RoleFactory {
    pub fn new() -> RoleFactory {
        RoleFactory {
            name: String::from("admin"),
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> RoleFactory {
        self.name = name.into();
        self
    }

    pub fn build(self) -> roles::ActiveModel {
        roles::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(self.name),
            ..Default::default()
        }
    }
}

pub struct PermissionFactory {
    name: String,
}

impl PermissionFactory {
    pub fn new() -> PermissionFactory {
        PermissionFactory {
            name: String::from("create"),
        }
    }

    pub fn name(mut self, name: String) -> PermissionFactory {
        self.name = name;
        self
    }

    pub fn build(self) -> permissions::ActiveModel {
        permissions::ActiveModel {
            id: Set(Uuid::new_v4()),
            name_permission: Set(self.name),
            ..Default::default()
        }
    }
}

pub struct UserFactory {
    pub id: Option<Uuid>,
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl UserFactory {
    pub fn new() -> UserFactory {
        UserFactory {
            id: Some(Uuid::new_v4()),
            name: String::from("John Doe"),
            username: String::from("john-doe"),
            email: String::from("john@example.com"),
            password: String::from("password"),
        }
    }

    pub fn id(mut self, id: Uuid) -> UserFactory {
        self.id = Some(id); self
    }

    pub fn name(mut self, name: String) -> UserFactory {
        self.name = name;
        self
    }

    pub fn username(mut self, username: String) -> UserFactory {
        self.username = username;
        self
    }

    pub fn email(mut self, email: String) -> UserFactory {
        self.email = email;
        self
    }

    pub fn password(mut self, password: String) -> UserFactory {
        self.password = password;
        self
    }

    pub fn build(self) -> users::ActiveModel {
        users::ActiveModel {
            id: Set(self.id.unwrap_or(Uuid::new_v4())),
            name: Set(self.name),
            username: Set(self.username),
            email: Set(self.email),
            password: Set(self.password),
            ..Default::default()
        }
    }

}

pub struct ProductFactory {
    id: Option<Uuid>,
    name: String,
    sku: String,
    slug: String,
    description: String,
    user_id: Uuid,
    price: Decimal,
    quantity: i64,
}

impl ProductFactory {
    pub fn new(admin_id: Uuid) -> ProductFactory {
        ProductFactory {
            id: Some(Uuid::new_v4()),
            name: String::from("Default Product"),
            sku: String::from("DEF-PROD-001"),
            slug: String::from("default-product"),
            description: String::from("Default product description"),
            user_id: admin_id,
            price: Decimal::new(1000, 2), // 10.00
            quantity: 10,
        }
    }

    pub fn id(mut self, id: Uuid) -> ProductFactory {
        self.id = Some(id);
        self
    }

    pub fn name(mut self, name: String) -> ProductFactory {
        self.name = name;
        self
    }

    pub fn sku(mut self, sku: String) -> ProductFactory {
        self.sku = sku;
        self
    }

    pub fn slug(mut self, slug: String) -> ProductFactory {
        self.slug = slug;
        self
    }

    pub fn description(mut self, description: String) -> ProductFactory {
        self.description = description;
        self
    }

    pub fn price(mut self, price: Decimal) -> ProductFactory {
        self.price = price;
        self
    }

    pub fn quantity(mut self, quantity: i64) -> ProductFactory {
        self.quantity = quantity;
        self
    }

    pub fn user_id(mut self, id: Uuid) -> ProductFactory {
        self.id = Some(id);
        self
    }

    pub fn build(self) -> products::ActiveModel {
        products::ActiveModel {
            id: Set(self.id.unwrap_or(Uuid::new_v4())),
            name: Set(self.name),
            sku: Set(self.sku),
            slug: Set(self.slug),
            description: Set(self.description),
            user_id: Set(self.user_id),
            price: Set(self.price),
            quantity: Set(self.quantity),
            ..Default::default()
        }
    }
}