#![allow(dead_code)]

use chrono::NaiveDateTime;

use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};

pub mod budget_management;

#[derive(Deserialize)]
pub struct RequestBudgetData {
    pub category: String,
    pub amount_budgeted: Decimal,
    pub amount_spent: Decimal,
}

#[derive(Serialize)]
pub struct ResponseBudgetData {
    id: i32,
    category: String,
    amount_budgeted: Decimal,
    amount_spent: Decimal,
    created_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct RequestUpdateBudgetData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_budgeted: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_spent: Option<Decimal>,
}
