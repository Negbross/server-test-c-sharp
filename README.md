Disc
: **List api endpoint di generate sama Gemini**

# Dummies API

Tester API documentation c sharp

## API Endpoints

### Authentication (`/auth`)

| Method | Endpoint         | Description           |
|--------|------------------|-----------------------|
| POST   | `/auth/login`    | Login user            |
| POST   | `/auth/register` | Register new user     |

---

### User Profile (`/user`)

| Method | Endpoint | Description         |
|--------|----------|---------------------|
| GET    | `/user/` | Get current user    |

---

### Products (`/products`)

| Method | Endpoint           | Description              |
|--------|--------------------|--------------------------|
| GET    | `/products/`       | Get all products         |
| POST   | `/products/`       | Create new product       |
| GET    | `/products/{slug}` | Get product by slug      |
| PUT    | `/products/{slug}` | Update product by slug   |
| DELETE | `/products/{slug}` | Delete product by slug   |

---

### Admin - User Management (`/admin/users`)

| Method | Endpoint                            | Description               |
|--------|-------------------------------------|---------------------------|
| GET    | `/admin/users/`                     | List all users            |
| GET    | `/admin/users/{id}`                 | Get user by ID            |
| PUT    | `/admin/users/{id}`                 | Update user by ID         |
| DELETE | `/admin/users/{id}`                 | Delete user by ID         |
| POST   | `/admin/users/{user_id}/roles/{role_id}` | Attach role to user  |

---

### Admin - Role Management (`/admin/roles`)

| Method | Endpoint            | Description        |
|--------|---------------------|--------------------|
| GET    | `/admin/roles/`     | List all roles     |
| POST   | `/admin/roles/`     | Create new role    |
| DELETE | `/admin/roles/{id}` | Delete role by ID  |

---

### Admin - Product Management (`/admin/products`)

| Method | Endpoint             | Description         |
|--------|----------------------|---------------------|
| GET    | `/admin/products/`   | List all products   |

---

## Documentation

- **Swagger UI**: `/api-docs`
- **OpenAPI JSON**: `/api-docs/openapi.json`