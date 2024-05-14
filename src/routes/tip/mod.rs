pub mod tip_management;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TipData {
    pub tip_message: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize)]
pub struct TipResponseData {
    pub tip_message: String,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct TipUpdateData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tip_message: Option<String>,
}
