use serde::{Deserialize, Serialize};
use crate::db::orders::Order;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct AlarmManagement {
    pub order_id: Option<i64>,
    pub fee_status: Option<String>,
    pub exceed_remark: Option<String>,
    pub delivery_remark: Option<String>,
    pub order: Option<Order>,
}
