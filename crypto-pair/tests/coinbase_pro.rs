mod utils;

use crypto_market_type::MarketType;
use crypto_pair::{get_market_type, normalize_currency, normalize_pair};
use serde::{Deserialize, Serialize};
use utils::http_get;

const EXCHANGE_NAME: &str = "coinbase_pro";

#[derive(Serialize, Deserialize)]
struct SpotMarket {
    id: String,
    base_currency: String,
    quote_currency: String,
}

// see <https://docs.pro.coinbase.com/#products>
fn fetch_spot_markets_raw() -> Vec<SpotMarket> {
    let txt = http_get("https://api.pro.coinbase.com/products").unwrap();
    serde_json::from_str::<Vec<SpotMarket>>(&txt)
        .unwrap()
        .into_iter()
        .filter(|m| !m.id.contains("AUCTION"))
        .collect()
}

#[test]
fn verify_spot_symbols() {
    let markets = fetch_spot_markets_raw();
    for market in markets.iter() {
        let pair = normalize_pair(&market.id, EXCHANGE_NAME).unwrap();
        let pair_expected = format!(
            "{}/{}",
            normalize_currency(&market.base_currency, EXCHANGE_NAME),
            normalize_currency(&market.quote_currency, EXCHANGE_NAME)
        );

        assert_eq!(pair.as_str(), pair_expected);
        assert_eq!(
            MarketType::Spot,
            get_market_type(&market.id, EXCHANGE_NAME, None)
        );
    }
}
