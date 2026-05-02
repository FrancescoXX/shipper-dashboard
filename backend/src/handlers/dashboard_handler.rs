use crate::state::AppState;
use axum::{extract::State, http::StatusCode, Json};
use shared::{DashboardStats, Member, RevenuePoint};
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

pub async fn members(
    State(state): State<AppState>,
) -> Result<Json<Vec<Member>>, (StatusCode, String)> {
    let rows = sqlx::query(
        r#"
        SELECT name, avatar_url, tier, joined_at
        FROM members
        ORDER BY joined_at DESC
        "#,
    )
    .fetch_all(&state.pool)
    .await
    .map_err(internal_error)?;

    let members = rows
        .into_iter()
        .map(|row| {
            let joined_at: chrono::DateTime<chrono::Utc> = row.get("joined_at");
            Member {
                name: row.get("name"),
                avatar_url: row.get("avatar_url"),
                tier: row.get("tier"),
                joined_at: joined_at.format("%b %-d, %Y").to_string(),
            }
        })
        .collect();

    Ok(Json(members))
}

fn internal_error(error: sqlx::Error) -> (StatusCode, String) {
    tracing::error!(%error, "database request failed");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "database request failed".to_string(),
    )
}
