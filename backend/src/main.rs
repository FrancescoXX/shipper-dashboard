mod app;
mod db;
mod handlers;
mod routes;
mod state;

use anyhow::Context;
use db::pool::create_pool;
use state::AppState;
use std::env;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=debug".into()),
        )
        .init();

    let database_url = env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
    let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string());

    let pool = create_pool(&database_url).await?;

    sqlx::migrate!("../migrations")
        .run(&pool)
        .await
        .context("failed to run database migrations")?;
    tracing::info!("migrations applied successfully");

    let state = AppState { pool };
    let router = app::router(state);

    let listener = TcpListener::bind(&bind_addr).await?;
    tracing::info!("backend listening on {bind_addr}");
    axum::serve(listener, router).await?;

    Ok(())
}
