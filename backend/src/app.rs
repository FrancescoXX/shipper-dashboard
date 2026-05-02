use crate::{routes, state::AppState};
use axum::Router;
use std::{env, sync::Arc};
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

const DEFAULT_RATE_LIMIT_PER_SECOND: u64 = 2;
const DEFAULT_RATE_LIMIT_BURST_SIZE: u32 = 30;

pub fn router(state: AppState) -> Router {
    let governor_config = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(env_u64(
                "RATE_LIMIT_PER_SECOND",
                DEFAULT_RATE_LIMIT_PER_SECOND,
            ))
            .burst_size(env_u32(
                "RATE_LIMIT_BURST_SIZE",
                DEFAULT_RATE_LIMIT_BURST_SIZE,
            ))
            .finish()
            .expect("rate limit config should be valid"),
    );

    Router::new()
        .nest("/api", routes::router())
        .layer(GovernorLayer {
            config: governor_config,
        })
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

fn env_u64(name: &str, default: u64) -> u64 {
    env::var(name)
        .ok()
        .and_then(|value| value.parse().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

fn env_u32(name: &str, default: u32) -> u32 {
    env::var(name)
        .ok()
        .and_then(|value| value.parse().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}
