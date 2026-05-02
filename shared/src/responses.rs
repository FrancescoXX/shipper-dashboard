use crate::{DashboardStats, RevenuePoint};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DashboardResponse {
    pub stats: DashboardStats,
    pub revenue: Vec<RevenuePoint>,
}
