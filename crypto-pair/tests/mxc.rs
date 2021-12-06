mod utils;

use crypto_market_type::MarketType;
use crypto_pair::{get_market_type, normalize_currency, normalize_pair};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use utils::http_get;

const EXCHANGE_NAME: &'static str = "mxc";

#[test]
fn verify_spot_symbols() {
    assert_eq!(
        "BTC/USDT".to_string(),
        normalize_pair("BTC_USDT", EXCHANGE_NAME).unwrap()
    );
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SwapMarket {
    symbol: String,
    baseCoin: String,
    quoteCoin: String,
    settleCoin: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    success: bool,
    code: i64,
    data: Vec<SwapMarket>,
}

// see <https://github.com/mxcdevelop/APIDoc/blob/master/contract/contract-api.md#contract-interface-public>
fn fetch_swap_markets_raw() -> Vec<SwapMarket> {
    let txt = http_get("https://contract.mexc.com/api/v1/contract/detail").unwrap();
    let resp = serde_json::from_str::<Response>(&txt).unwrap();
    resp.data
}

#[test]
fn verify_swap_symbols() {
    let markets = fetch_swap_markets_raw();
    for market in markets.iter() {
        let pair = normalize_pair(&market.symbol, EXCHANGE_NAME).unwrap();
        let pair_expected = format!(
            "{}/{}",
            normalize_currency(&market.baseCoin, EXCHANGE_NAME),
            normalize_currency(&market.quoteCoin, EXCHANGE_NAME)
        );

        assert_eq!(pair.as_str(), pair_expected);

        let market_type = get_market_type(&market.symbol, EXCHANGE_NAME, None);
        assert!(market_type == MarketType::LinearSwap || market_type == MarketType::InverseSwap);
    }
}
