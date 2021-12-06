mod utils;

use crypto_market_type::MarketType;
use crypto_pair::{get_market_type, normalize_pair};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use utils::http_get;

const EXCHANGE_NAME: &'static str = "bitstamp";

#[derive(Serialize, Deserialize)]
struct SpotMarket {
    name: String,
    trading: String,
    url_symbol: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// see <https://www.bitstamp.net/api/>
fn fetch_spot_markets_raw() -> Vec<SpotMarket> {
    let txt = http_get("https://www.bitstamp.net/api/v2/trading-pairs-info/").unwrap();
    serde_json::from_str::<Vec<SpotMarket>>(&txt).unwrap()
}

#[test]
fn verify_spot_symbols() {
    let markets = fetch_spot_markets_raw();
    for market in markets.iter() {
        let pair = normalize_pair(&market.url_symbol, EXCHANGE_NAME).unwrap();
        let pair_expected = market.name.as_str();

        assert_eq!(pair.as_str(), pair_expected);
        assert_eq!(
            MarketType::Spot,
            get_market_type(&market.url_symbol, EXCHANGE_NAME, None)
        );
    }
}
