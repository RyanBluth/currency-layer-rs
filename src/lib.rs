//! currency-layer-rs is a simple client for accesing the free APIs at https://currencylayer.com/

#[macro_use]
extern crate serde_derive;

#[macro_use] 
extern crate failure;

extern crate serde;
extern crate serde_json;

extern crate reqwest;

mod types;

pub use types::CurrencyRates;
use types::*;

use std::collections::HashMap;
use std::io::Read;

use failure::Error;

/// Currency Layer errors
#[derive(Debug, Fail)]
pub enum CurrencyLayerError{
    
    /// This error is occur if an invalid currency symbol is provided
    #[fail(display = "Invalid currency symbol: {}", symbol)]
    InvalidCurrency{
        /// The invalid currency symbol
        symbol: String
    },

    /// This error will occure if Currency Layer returns an error response
    #[fail(display = "Currency Layer responded with an error: Code: {}. Message: {}", code, message)]
    ServerError{
        /// The error code returned in the message body
        code: u16,
        /// The returned error message
        message: String
    }
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

    /// Get the excahnge rates for the provide currencies.
    /// 
    /// All values are relative to the base currency.
    pub fn get_live_rates(
        &self,
        base: &str,
        currencies: Vec<&str>,
    ) -> Result<CurrencyRates, Error> {
        return self.get_rates(base, currencies, None, "http://apilayer.net/api/live");
    }

    /// Get the excahnge rates for the provide currencies on a paticular day.
    /// 
    /// All values are relative to the base currency.
    /// 
    /// date is a tuple 3 in the format year, month, day.
    pub fn get_historical_rates(
        &self,
        base: &str,
        currencies: Vec<&str>,
        date: (u16, u16, u16),
    ) -> Result<CurrencyRates, Error> {
    
        return self.get_rates(base, currencies, Some(date), "http://apilayer.net/api/historical");
    }

    fn get_rates(
        &self,
        base: &str,
        currencies: Vec<&str>,
        date: Option<(u16, u16, u16)>,
        url: &str,
    ) -> Result<CurrencyRates, Error> {

        let mut mut_currencies = currencies.clone();

        mut_currencies.push(base);

        let currencies_str = mut_currencies.join(",");

        let mut query_items = vec![
                ("currencies", currencies_str),
                ("format", "1".into()),
                ("access_key", self.key.clone()),
            ];
        
        if let Some(d) = date{
            query_items.push(("date", format!("{}-{:02}-{:02}", d.0, d.1, d.2)));
        }

        let request = self.http_client
            .get(url)
            .query(&query_items)
            .send();

        let mut res = request?;

        let mut body_buf = String::new();

        res.read_to_string(&mut body_buf)?;

        let success_guard: SuccessGuard = serde_json::from_str(body_buf.as_str())?;

        if success_guard.success {
            let result: CurrencyRates = serde_json::from_str(body_buf.as_str())?;

            let base_val = 1.0 / result.quotes.get(&format!("USD{}", base)).unwrap();

            let mut res = CurrencyRates {
                timestamp: result.timestamp,
                quotes: HashMap::new(),
            };

            for c in currencies {
                if let Some(quote) = result.quotes.get(&format!("USD{}", c)) {
                    res.quotes.insert(String::from(c), base_val * quote);
                } else {
                    return Err(CurrencyLayerError::InvalidCurrency {
                        symbol: String::from(c)
                    }.into());
                }
            }
            return Ok(res);
        }else {
            let result: ErrorResponse = serde_json::from_str(body_buf.as_str())?;
            return Err(CurrencyLayerError::ServerError {
                code: result.error.code,
                message: result.error.info
            }.into())
        }
    }
}
