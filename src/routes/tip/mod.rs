pub mod tip_management;

use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TipData {
    pub tip_message: String,
    pub created_at: NaiveDateTime,
}
