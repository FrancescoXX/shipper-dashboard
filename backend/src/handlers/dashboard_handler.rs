use crate::state::AppState;
use axum::{extract::State, http::StatusCode, Json};
use shared::{BatchSummary, DashboardStats, RevenuePoint};
use sqlx::Row;

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
) -> Result<Json<Vec<RevenuePoint>>, (StatusCode, String)> {
    let rows = sqlx::query(
        r#"
        SELECT date, revenue_cents
        FROM revenue_points
        ORDER BY date ASC
        "#,
    )
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

pub async fn batch_summary(
    State(state): State<AppState>,
) -> Result<Json<BatchSummary>, (StatusCode, String)> {
    let row = sqlx::query(
        r#"
        SELECT
            batch_number,
            total_batches,
            claimed_spots,
            total_spots,
            spots_left,
            tier_name,
            founder_name,
            founder_avatar_url,
            cta_url
        FROM batch_summary
        ORDER BY id DESC
        LIMIT 1
        "#,
    )
    .fetch_one(&state.pool)
    .await
    .map_err(internal_error)?;

    Ok(Json(BatchSummary {
        batch_number: row.get("batch_number"),
        total_batches: row.get("total_batches"),
        claimed_spots: row.get("claimed_spots"),
        total_spots: row.get("total_spots"),
        spots_left: row.get("spots_left"),
        tier_name: row.get("tier_name"),
        founder_name: row.get("founder_name"),
        founder_avatar_url: row.get("founder_avatar_url"),
        cta_url: row.get("cta_url"),
    }))
}

fn internal_error(error: sqlx::Error) -> (StatusCode, String) {
    tracing::error!(%error, "database request failed");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "database request failed".to_string(),
    )
}
