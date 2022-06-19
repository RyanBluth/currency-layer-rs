extern crate currency_layer;
extern crate tokio;

use currency_layer::Client;
use rusty_money::{iso, Money};

#[tokio::main]
async fn main() {
    let client = Client::new("API_KEY");
    let result = client
        .get_live_rates(
            // Currencies to get rates for
            vec!["GBP", "USD", "CAD"],
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
    // At 2021-02-16 02:12:06 UTC: $100 → £71.74
    println!(
        "At {}: {} → {}",
        result.timestamp, &original_usd, &converted_to_gbp
    );
}
