pub mod transaction_management;

use chrono::NaiveDate;
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TransactionData {
    pub balance: Decimal,
    pub category: String,
}

#[derive(Serialize)]
pub struct ResponseTransactionData {
    id: i32,
    balance: Decimal,
    category: String,
    update_at: NaiveDate,
}

#[derive(Deserialize)]
pub struct TransactionUpdateData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}
