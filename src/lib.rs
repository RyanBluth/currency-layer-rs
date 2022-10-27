//! currency-layer-rs is a simple client for accesing the free APIs at https://currencylayer.com/
mod types;

use thiserror::Error;
use rusty_money::{iso, ExchangeRate};
use std::collections::HashMap;
pub use types::CurrencyRates;
use types::*;

/// Currency Layer errors
#[derive(Debug, Error)]
pub enum CurrencyLayerError {
    /// This error is occur if an invalid currency symbol is provided
    #[error("Invalid currency symbol: {}", symbol)]
    InvalidCurrency {
        /// The invalid currency symbol
        symbol: String,
    },

    /// This error will occure if Currency Layer returns an error response
    #[error(
        "Currency Layer responded with an error: Code: {}. Message: {}",
        code, message
    )]
    ServerError {
        /// The error code returned in the message body
        code: u16,
        /// The returned error message
        message: String,
    },

    #[error(transparent)]
    MoneyError(#[from] rusty_money::MoneyError),

    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

/// Client for making requests to Currency Layer
pub struct Client {
    /// API Key which can be optained from the Currency Layer website https://currencylayer.com/
    key: String,

    /// Reqwest client
    http_client: reqwest::Client,
}

impl Client {
    /// Creates a new client using the provided key for all requests.
    ///
    /// Only the free APIs are supported.
    pub fn new(key: &str) -> Client {
        Client {
            key: String::from(key),
            http_client: reqwest::Client::new(),
        }
    }

    /// Get the exchange rates for the provide currencies.
    pub async fn get_live_rates(&self, currencies: Vec<&str>) -> Result<CurrencyRates<'_>, CurrencyLayerError> {
        self.get_rates(currencies, None, "http://apilayer.net/api/live")
            .await
    }

    /// Get the exchange rates for the provide currencies on a particular day.
    ///
    /// date is a tuple 3 in the format year, month, day.
    pub async fn get_historical_rates(
        &self,
        currencies: Vec<&str>,
        date: (u16, u16, u16),
    ) -> Result<CurrencyRates<'_>, CurrencyLayerError> {
        self.get_rates(currencies, Some(date), "http://apilayer.net/api/historical")
            .await
    }

    async fn get_rates(
        &self,
        currencies: Vec<&str>,
        date: Option<(u16, u16, u16)>,
        url: &str,
    ) -> Result<CurrencyRates<'_>, CurrencyLayerError> {
        let mut query_items = vec![
            ("currencies", currencies.join(",")),
            ("format", "1".into()),
            ("access_key", self.key.clone()),
        ];

        if let Some(d) = date {
            query_items.push(("date", format!("{}-{:02}-{:02}", d.0, d.1, d.2)));
        }

        let response = self.http_client.get(url).query(&query_items).send().await?;

        let body_buf = response.text().await?;

        let success_guard: SuccessGuard = serde_json::from_str(&body_buf)?;
        if !success_guard.success {
            let result: ErrorResponse = serde_json::from_str(&body_buf)?;
            return Err(CurrencyLayerError::ServerError {
                code: result.error.code,
                message: result.error.info,
            }
            .into());
        }

        let result: CurrencyRatesResponse = serde_json::from_str(&body_buf)?;

        fn lookup(symbol: &str) -> Result<&'static iso::Currency, CurrencyLayerError> {
            iso::find(symbol).ok_or(
                CurrencyLayerError::InvalidCurrency {
                    symbol: symbol.to_string().clone(),
                }
                .into(),
            )
        }

        let quotes: HashMap<String, ExchangeRate<iso::Currency>> = result
            .quotes
            .into_iter()
            .map(|(symbol_pair, rate)| {
                let (from_symbol, to_symbol) = symbol_pair.split_at(3);
                (
                    (
                        from_symbol.to_string().clone(),
                        to_symbol.to_string().clone(),
                    ),
                    rate,
                )
            })
            // Drop "USDUSD" - ExchangeRate::new is not happy with it.
            .filter(|((from_symbol, to_symbol), _rate)| from_symbol != to_symbol)
            .map(|((from_symbol, to_symbol), rate)| {
                let from = lookup(&from_symbol)?;
                let to = lookup(&to_symbol)?;
                Ok((to_symbol, ExchangeRate::new(from, to, rate)?))
            })
            .collect::<Result<_, CurrencyLayerError>>()?;
        Ok(CurrencyRates {
            timestamp: result.timestamp,
            quotes,
        })
    }
}
