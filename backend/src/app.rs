use crate::{routes, state::AppState};
use axum::Router;
use tower::limit::ConcurrencyLimitLayer;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub fn router(state: AppState) -> Router {
    Router::new()
        .nest("/api", routes::router())
        .layer(ConcurrencyLimitLayer::new(100))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
