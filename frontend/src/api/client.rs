use reqwasm::http::Request;
use shared::{DashboardStats, Member, RevenuePoint};

const API_BASE: &str = "/api";

pub async fn fetch_stats() -> Result<DashboardStats, String> {
    fetch_json(&format!("{API_BASE}/dashboard/stats")).await
}

pub async fn fetch_revenue() -> Result<Vec<RevenuePoint>, String> {
    fetch_json(&format!("{API_BASE}/dashboard/revenue")).await
}

pub async fn fetch_members() -> Result<Vec<Member>, String> {
    fetch_json(&format!("{API_BASE}/dashboard/members")).await
}

async fn fetch_json<T>(url: &str) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
{
    let response = Request::get(url)
        .send()
        .await
        .map_err(|error| format!("request failed: {error}"))?;

    if !response.ok() {
        return Err(format!("request failed with status {}", response.status()));
    }

    response
        .json()
        .await
        .map_err(|error| format!("invalid response: {error}"))
}
