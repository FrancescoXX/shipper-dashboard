use crate::state::AppState;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDate;
use serde::Deserialize;
use shared::{DashboardStats, RevenuePoint};
use sqlx::Row;

#[derive(Deserialize)]
pub struct RevenueQuery {
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
}

pub async fn stats(
    State(state): State<AppState>,
) -> Result<Json<DashboardStats>, (StatusCode, String)> {
    let row = sqlx::query(
        r#"
        SELECT
            total_members,
            current_tier,
            claimed_spots,
            spots_left,
            estimated_revenue_cents
        FROM dashboard_stats
        ORDER BY id DESC
        LIMIT 1
        "#,
    )
    .fetch_one(&state.pool)
    .await
    .map_err(internal_error)?;

    Ok(Json(DashboardStats {
        total_members: row.get("total_members"),
        current_tier: row.get("current_tier"),
        claimed_spots: row.get("claimed_spots"),
        spots_left: row.get("spots_left"),
        estimated_revenue_cents: row.get("estimated_revenue_cents"),
    }))
}

pub async fn revenue(
    State(state): State<AppState>,
    Query(params): Query<RevenueQuery>,
) -> Result<Json<Vec<RevenuePoint>>, (StatusCode, String)> {
    let today = chrono::Utc::now().date_naive();
    let from = params
        .from
        .unwrap_or_else(|| today - chrono::Duration::days(30));
    let to = params.to.unwrap_or(today);

    let rows = sqlx::query(
        r#"
        SELECT date, revenue_cents
        FROM revenue_points
        WHERE date >= $1 AND date <= $2
        ORDER BY date ASC
        "#,
    )
    .bind(from)
    .bind(to)
    .fetch_all(&state.pool)
    .await
    .map_err(internal_error)?;

    let points = rows
        .into_iter()
        .map(|row| RevenuePoint {
            date: row.get("date"),
            revenue_cents: row.get("revenue_cents"),
        })
        .collect();

    Ok(Json(points))
}

fn internal_error(error: sqlx::Error) -> (StatusCode, String) {
    tracing::error!(%error, "database request failed");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "database request failed".to_string(),
    )
}
