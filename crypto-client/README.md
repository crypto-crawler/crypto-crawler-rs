# crypto-client

An unified client for all cryptocurrency exchanges.

## Example

```rust
use crypto_client::{CryptoClient, MarketType};

fn main() {
    let config: HashMap<&str, &str> = vec![
        ("eosAccount", "your-eos-account"),
        ("eosPrivateKey", "your-eos-private-key"),
    ].into_iter().collect();

    let crypto_client = CryptoClient::new(config);

    // buy
    let transaction_id = crypto_client.place_order(
        { exchange: "Newdex", pair: "EIDOS_EOS", market_type: "Spot" },
        0.00121,
        9.2644,
        false,
    );
    println!("{}", transactionId);
}
```

## Supported Exchanges

- Binance
- Huobi
- OKEx
- WhaleEx
