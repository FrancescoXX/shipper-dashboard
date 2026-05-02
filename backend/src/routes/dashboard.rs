use crate::{handlers::dashboard_handler, state::AppState};
use axum::{routing::get, Router};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/stats", get(dashboard_handler::stats))
        .route("/revenue", get(dashboard_handler::revenue))
        .route("/members", get(dashboard_handler::members))
}
