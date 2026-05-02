use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DashboardStats {
    pub total_members: i64,
    pub current_tier: String,
    pub claimed_spots: i64,
    pub spots_left: i64,
    pub estimated_revenue_cents: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RevenuePoint {
    pub date: NaiveDate,
    pub revenue_cents: i64,
}
