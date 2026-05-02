# Contributing to Shipper Dashboard

Thank you for your interest in contributing! Please follow these guidelines to keep the process smooth.

## Issue-First Workflow

1. **Create an issue** before starting any work — describe the problem or feature.
2. **Wait for approval** from a maintainer before opening a pull request.
3. If an issue already exists, comment on it to signal you'd like to work on it.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable)
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- [Trunk](https://trunkrs.dev/) for frontend: `cargo install trunk`
- [PostgreSQL](https://www.postgresql.org/) running locally
- Copy `.env.example` to `.env` and set your `DATABASE_URL`

### Running Locally

```bash
# Backend
cargo run -p backend

# Frontend (in a separate terminal)
cd frontend && trunk serve
```

## Branch Naming

Use a prefix that matches the type of change:

| Prefix     | Example                          |
|------------|----------------------------------|
| `feat/`    | `feat/dynamic-batch-summary`     |
| `fix/`     | `fix/health-db-check`            |
| `chore/`   | `chore/auto-migrate`             |
| `docs/`    | `docs/contributing`              |
| `test/`    | `test/api-integration`           |
| `ci/`      | `ci/github-actions`              |

## Commit Messages

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add batch summary endpoint
fix: health endpoint verifies database connectivity
chore: enable sqlx migrate feature
docs: add CONTRIBUTING.md
test: add integration tests for stats endpoint
ci: add GitHub Actions workflow
```

## Code Quality

Before submitting a PR, make sure:

```bash
# Format
cargo fmt --all

# Lint (backend)
cargo clippy -p shared -p backend -- -D warnings

# Check backend
cargo check -p shared -p backend

# Check frontend
cargo check -p frontend --target wasm32-unknown-unknown
```

## Pull Request Checklist

- [ ] References an approved issue (e.g. `Closes #1`)
- [ ] Follows branch naming convention
- [ ] Uses conventional commit messages
- [ ] `cargo fmt` passes
- [ ] `cargo check` passes for affected crates
- [ ] No unrelated changes included
- [ ] Description explains *what* and *why*

## Project Structure

```
shipper-dashboard/
├── backend/          # Axum API server
├── frontend/         # Leptos CSR app (built with Trunk)
├── shared/           # Shared models between backend & frontend
├── migrations/       # SQL migration files
├── docker-compose.yml
└── Cargo.toml        # Workspace root
```

## Questions?

Open a discussion or issue — we're happy to help!
