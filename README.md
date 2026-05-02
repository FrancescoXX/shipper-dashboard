# Shipper Club Dashboard MVP

Minimal Rust full-stack dashboard for Shipper Club.

<img width="1522" height="1184" alt="image" src="https://github.com/user-attachments/assets/1e50a511-94b2-4013-9531-ca747f23038d" />

## Stack

- Frontend: Leptos CSR, built with Trunk
- Backend: Axum
- Shared types: Rust `shared` crate
- Database: PostgreSQL
- Database access: SQLx
- Local infrastructure: Docker Compose

<img width="1448" height="1086" alt="image" src="https://github.com/user-attachments/assets/85ca6e6d-b19b-4b2f-adc3-4087df3ca6e0" />


## Project Layout

```text
frontend/    Leptos dashboard app
backend/     Axum JSON API
shared/      Shared API models and responses
migrations/  SQLx migrations and seed data
```

## API

- `GET /api/health`
- `GET /api/dashboard/stats`
- `GET /api/dashboard/revenue`

## Local Setup

Copy the sample environment file:

```sh
cp .env.example .env
```

Start PostgreSQL:

```sh
docker compose up -d postgres
```

Install SQLx CLI if needed:

```sh
cargo install sqlx-cli --no-default-features --features postgres --locked
```

Run migrations and seed data:

```sh
sqlx migrate run
```

Start the backend:

```sh
cargo run -p backend
```

Start the frontend in another terminal:

```sh
cd frontend
trunk serve
```

Open `http://localhost:8080`.

## Docker Compose

Run the full project:

```sh
docker compose up --build
```

Then open:

- Frontend: `http://localhost:8080`
- Backend health: `http://localhost:3000/api/health`

The compose setup starts PostgreSQL, runs SQLx migrations, starts the backend, and serves the frontend through Nginx with `/api` proxied to the backend.

## Notes

- Data is seeded by `migrations/002_seed_initial_data.sql`.
- There is no authentication, payment integration, Discord integration, or scraping in this MVP.
- The frontend intentionally uses simple CSS and a minimal bar chart to keep the first version easy to change.
# shipper-dashboard
