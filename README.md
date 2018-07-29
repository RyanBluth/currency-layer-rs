### currency-layer-rs is a client for [CurrencyLayer's](https://currencylayer.com/) free APIs

## Example Usage

### Live Rates
```rust
    extern crate currency_layer

    use currency_layer::Client

    fn main(){
        let client = Client::new("YOU_API_KEY");
        let res = client.get_live_rates(
            // Base currency that results are relative to
            "CAD", 
            // Currencies to get rates for
            vec!["GBP", "USD"]
        );
        println!("{}", res.quotes.get("GBP").unwrap());
        println!("{}", res.quotes.get("USD").unwrap());
    }
```

### Historical Rates
```rust
    extern crate currency_layer

    use currency_layer::Client

    fn main(){
        let client = Client::new("YOU_API_KEY");
        let res = client.get_historical_rates(
            // Base currency that results are relative to
            "CAD", 
            // Currencies to get rates for
            vec!["GBP", "USD"],
            // Date as a tuple 3 in the format YYYY, MM, DD
            (2015, 2, 23)
        );

        println!("{}", res.quotes.get("GBP").unwrap());
        println!("{}", res.quotes.get("USD").unwrap());
    }
```

