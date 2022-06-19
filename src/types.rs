use chrono::{serde::ts_seconds, DateTime, Utc};
use rust_decimal::prelude::Decimal;
use rusty_money::{iso::Currency, ExchangeRate};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct SuccessGuard {
    pub success: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CurrencyRatesResponse {
    /// Time of the request
    #[serde(with = "ts_seconds")]
    pub timestamp: DateTime<Utc>,

    /// Currency rates keyed by currency code
    pub quotes: HashMap<String, Decimal>,
}

#[derive(Debug, Clone)]
pub struct CurrencyRates<'a> {
    /// Time of the request
    pub timestamp: DateTime<Utc>,

    /// Currency rates keyed by target currency code
    pub quotes: HashMap<String, ExchangeRate<'a, Currency>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ErrorBody {
    pub code: u16,
    pub info: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorBody,
}
