use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct SuccessGuard {
    pub success: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CurrencyRates {
    /// Time of the request
    pub timestamp: usize,

    /// Currency rates keyed by currency code
    pub quotes: HashMap<String, f64>,
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
