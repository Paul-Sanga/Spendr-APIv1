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
    category: String,
    amount_budgeted: Decimal,
    amount_spent: Decimal,
    created_at: NaiveDateTime,
}
