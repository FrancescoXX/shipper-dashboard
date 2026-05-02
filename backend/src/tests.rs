#[cfg(test)]
mod tests {
    use crate::{app, db::pool::create_pool, state::AppState};
    use axum::http::{Request, StatusCode};
    use http_body_util::BodyExt;
    use shared::{DashboardStats, RevenuePoint};
    use tower::ServiceExt;

    async fn test_app() -> axum::Router {
        dotenvy::dotenv().ok();
        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");
        let pool = create_pool(&database_url)
            .await
            .expect("failed to create test pool");
        app::router(AppState { pool })
    }

    async fn get_body(app: axum::Router, uri: &str) -> (StatusCode, Vec<u8>) {
        let response = app
            .oneshot(
                Request::builder()
                    .uri(uri)
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let status = response.status();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        (status, body.to_vec())
    }

    #[tokio::test]
    async fn health_returns_ok() {
        let app = test_app().await;
        let (status, body) = get_body(app, "/health").await;
        assert_eq!(status, StatusCode::OK);
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["status"], "ok");
    }

    #[tokio::test]
    async fn stats_returns_dashboard_stats() {
        let app = test_app().await;
        let (status, body) = get_body(app, "/api/dashboard/stats").await;
        assert_eq!(status, StatusCode::OK);
        let stats: DashboardStats = serde_json::from_slice(&body).unwrap();
        assert!(stats.total_members > 0);
        assert!(!stats.current_tier.is_empty());
    }

    #[tokio::test]
    async fn revenue_returns_points() {
        let app = test_app().await;
        let (status, body) = get_body(app, "/api/dashboard/revenue").await;
        assert_eq!(status, StatusCode::OK);
        let points: Vec<RevenuePoint> = serde_json::from_slice(&body).unwrap();
        assert!(!points.is_empty());
        assert!(points[0].revenue_cents > 0);
    }

    #[tokio::test]
    async fn unknown_route_returns_404() {
        let app = test_app().await;
        let (status, _) = get_body(app, "/api/does-not-exist").await;
        assert_eq!(status, StatusCode::NOT_FOUND);
    }
}
