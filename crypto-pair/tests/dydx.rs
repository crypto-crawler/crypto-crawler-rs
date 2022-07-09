mod utils;

use crypto_market_type::MarketType;
use crypto_pair::{get_market_type, normalize_currency, normalize_pair};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use utils::http_get;

const EXCHANGE_NAME: &str = "dydx";

#[derive(Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct PerpetualMarket {
    market: String,
    status: String,
    baseAsset: String,
    quoteAsset: String,
    #[serde(rename = "type")]
    type_: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct MarketsResponse {
    markets: HashMap<String, PerpetualMarket>,
}

// See https://docs.dydx.exchange/#get-markets
fn fetch_markets_raw() -> Vec<PerpetualMarket> {
    let txt = http_get("https://api.dydx.exchange/v3/markets").unwrap();
    let resp = serde_json::from_str::<MarketsResponse>(&txt).unwrap();
    resp.markets
        .values()
        .cloned()
        .filter(|x| x.status == "ONLINE")
        .collect::<Vec<PerpetualMarket>>()
}

#[test]
fn verify_linear_swap_symbols() {
    let markets = fetch_markets_raw();
    for market in markets {
        let pair = normalize_pair(&market.market, EXCHANGE_NAME).unwrap();
        let pair_expected = format!(
            "{}/{}",
            normalize_currency(&market.baseAsset, EXCHANGE_NAME),
            normalize_currency(&market.quoteAsset, EXCHANGE_NAME)
        );

        assert_eq!(pair, pair_expected);
        assert_eq!(
            MarketType::LinearSwap,
            get_market_type(&market.market, EXCHANGE_NAME, None)
        );
    }
}
