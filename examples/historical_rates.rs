extern crate currency_layer;
extern crate tokio;

use currency_layer::Client;
use rusty_money::{iso, Money};

#[tokio::main]
async fn main() {
    let client = Client::new("API_KEY");
    let result = client
        .get_historical_rates(
            // Currencies to get rates for
            vec!["GBP", "USD", "CAD"],
            // Date as a tuple 3 in the format YYYY, MM, DD
            (2015, 2, 23),
        )
        .await
        .unwrap();

    // The free API only returns rates in dollars.
    // See: https://currencylayer.com/documentation, section
    // "Source Currency Switching".
    let original_usd = Money::from_major(100, iso::USD);
    let exchange_rate_usd_to_gbp = result.quotes.get("GBP").unwrap();
    let converted_to_gbp = exchange_rate_usd_to_gbp
        .convert(original_usd.clone())
        .unwrap();
    // At 2015-02-23 23:59:59 UTC: $100 → £64.70
    println!(
        "At {}: {} → {}",
        result.timestamp, &original_usd, &converted_to_gbp
    );
}
