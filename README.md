# Backend RUST Starter with JWT Authentication

Backend API menggunakan Rust dengan Axum framework, MySQL database, JWT authentication, dan role-based access control (RBAC).

## Features

- ✅ JWT Authentication
- ✅ Role-Based Access Control (RBAC)
- ✅ User Management (CRUD)
- ✅ Modular Handler Structure
- ✅ Docker Support
- ✅ MySQL Database
- ✅ Password Hashing with Bcrypt
- ✅ Input Validation
- ✅ CORS Support

## Database Schema

### Users Table
- `id` - Primary key (INT UNSIGNED AUTO_INCREMENT)
- `name` - User's full name
- `email` - Unique email address
- `email_verified_at` - Email verification timestamp
- `password` - Hashed password
- `status` - User status (active, inactive, etc.)
- `uid` - Unique identifier (UUID)
- `role_id` - Foreign key to roles table (default: '3')
- `username` - Optional username
- `remember_token` - Remember me token
- `users_token` - User verification token
- `created_at` - Creation timestamp
- `updated_at` - Last update timestamp

### Roles Table (tbl_roles)
- `tbl_roles_id` - Primary key
- `name` - Role name
- `display_name` - Display name
- `description` - Role description
- `status` - Active status (1 = active, 0 = inactive)
- `created_at` - Creation timestamp
- `updated_at` - Last update timestamp

## Instalasi

### Metode 1: Docker (Recommended)

1. **Clone repository**
```bash
git clone <repository-url>
cd backend-api-jwt
```

2. **Jalankan dengan Docker Compose**
```bash
docker-compose up -d
```

3. **Cek status container**
```bash
docker-compose ps
```

4. **Lihat logs**
```bash
docker-compose logs -f backend
```

5. **Stop containers**
```bash
docker-compose down
```

6. **Stop dan hapus volumes**
```bash
docker-compose down -v
```

### Metode 2: Manual Installation

1. **Install Rust**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. **Verifikasi instalasi**
```bash
rustc --version
cargo --version
```

3. **Install Dependencies**
```bash
cargo install cargo-watch
cargo install sqlx-cli
```

4. **Setup Database**

Pastikan MySQL sudah terinstall dan berjalan, kemudian buat database:
```sql
CREATE DATABASE backend_api_jwt;
```

5. **Setup Environment Variables**

Copy `.env` file dan sesuaikan konfigurasi:
```bash
cp .env.example .env
```

Edit `.env`:
```
APP_HOST=localhost
APP_PORT=3000
DATABASE_URL=mysql://root:secret@localhost:3306/backend_api_jwt
JWT_SECRET=bcrandomsecretkeyforjwt1234567890
```

6. **Jalankan Migrasi**
```bash
sqlx migrate run
```

7. **Jalankan Aplikasi**

Development mode dengan auto-reload:
```bash
cargo watch -q -c -w src/ -x run
```

Production mode:
```bash
cargo run --release
```

## API Endpoints

### Authentication (Public)
- `POST /api/register` - Register user baru
- `POST /api/login` - Login user

### Users (Protected)
- `GET /api/users` - List semua user
- `POST /api/users` - Tambah user baru
- `GET /api/users/:id` - Detail user
- `PUT /api/users/:id` - Update user
- `DELETE /api/users/:id` - Hapus user

### Roles (Protected)
- `GET /api/roles` - List semua role

## Project Structure

```
backend-api-jwt/
├── src/
│   ├── config/           # Database configuration
│   ├── handlers/         # Request handlers (modular)
│   │   ├── auth/        # Authentication handlers
│   │   ├── user_management/  # User CRUD handlers
│   │   └── role/        # Role handlers
│   ├── middlewares/      # Auth middleware
│   ├── models/          # Data models
│   ├── routes/          # Route definitions
│   ├── schemas/         # Request/Response schemas
│   ├── utils/           # Utilities (JWT, Response)
│   └── main.rs          # Application entry point
├── migrations/          # Database migrations
├── Dockerfile           # Docker configuration
├── docker-compose.yml   # Docker Compose configuration
└── Cargo.toml          # Rust dependencies

```

## Development

### Watch Mode
```bash
cargo watch -q -c -w src/ -x run
```

Flags:
- `-q`: Quiet mode
- `-c`: Clear screen on reload
- `-w src/`: Watch src/ directory
- `-x run`: Execute cargo run

### Check Compilation
```bash
cargo check
```

### Run Tests
```bash
cargo test
```

## Docker Commands

### Build image
```bash
docker-compose build
```

### Rebuild without cache
```bash
docker-compose build --no-cache
```

### View logs
```bash
docker-compose logs -f
```

### Execute command in container
```bash
docker-compose exec backend sh
```

### Build migrations in Docker
```bash
docker-compose exec backend sqlx migrate run
```

### Note on Docker Build (Runtime Queries)

Project ini telah dikonfigurasi menggunakan runtime queries (`sqlx::query` & `sqlx::query_as`). Artinya, Anda **tidak memerlukan** database yang berjalan saat melakukan `docker-compose up --build`. 

`sqlx-cli` juga sudah terinstall di dalam docker image builder untuk membantu manajemen database jika diperlukan.

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| APP_HOST | Server host | localhost |
| APP_PORT | Server port | 3000 |
| DATABASE_URL | MySQL connection string | - |
| JWT_SECRET | Secret key for JWT | - |

## Authentication

API menggunakan JWT (JSON Web Token) untuk autentikasi. Setelah login, gunakan token di header:

```
Authorization: Bearer <your-jwt-token>
```

Token berisi informasi:
- `sub`: User ID
- `role_id`: Role ID untuk RBAC
- `exp`: Expiration time (24 jam)

## Default Roles

- Role ID 1: Administrator (Full access)
- Role ID 2: Regular User (Default untuk user baru)

## License

MIT




