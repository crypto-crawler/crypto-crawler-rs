# crypto-rest-client

A restful client for all cryptocurrency exchanges.

## Example

```rust
use crypto_rest_client::{BinanceClient};

fn main() {
    let config: HashMap<&str, &str> = vec![
        ("api_key", "your-API-key"),
        ("api_secret", "your-API-secret"),
    ].into_iter().collect();

    let rest_client = BinanceClient::new(config);

    // buy
    let transaction_id = rest_client.place_order("Spot", "btcusdt", 27999.9, 5.0, false);
    println!("{}", transactionId);
}
```

## Supported Exchanges

-   Binance
-   Huobi
-   OKEx
