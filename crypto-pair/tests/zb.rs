mod utils;

use crypto_market_type::MarketType;
use crypto_pair::{get_market_type, normalize_pair};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use utils::http_get;

const EXCHANGE_NAME: &str = "zb";

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SwapMarket {
    symbol: String,
    marginCurrencyName: String,
    buyerCurrencyName: String,
    sellerCurrencyName: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Response {
    code: i64,
    data: Vec<SwapMarket>,
}

// See https://github.com/ZBFuture/docs/blob/main/API%20V2%20_en.md#71-trading-pair
fn fetch_swap_markets(url: &str) -> Vec<SwapMarket> {
    let txt = http_get(url).unwrap();
    let resp = serde_json::from_str::<Response>(&txt).unwrap();
    if resp.code == 1000 {
        resp.data
    } else {
        Vec::new()
    }
}

fn fetch_swap_markets_all() -> Vec<SwapMarket> {
    let usdt_markets = fetch_swap_markets("https://fapi.zb.com/Server/api/v2/config/marketList");
    let qc_markets = fetch_swap_markets("https://fapi.zb.com/qc/Server/api/v2/config/marketList");
    usdt_markets
        .into_iter()
        .chain(qc_markets.into_iter())
        .collect()
}

#[test]
fn verify_spot_symbols() {
    assert_eq!(
        Some("BTC/USDT".to_string()),
        normalize_pair("btc_usdt", EXCHANGE_NAME)
    );
    assert_eq!(
        Some("BTC/USDT".to_string()),
        normalize_pair("btcusdt", EXCHANGE_NAME)
    );
}

#[test]
fn verify_swap_symbols() {
    let markets = fetch_swap_markets_all();
    for market in markets.iter() {
        let pair = normalize_pair(&market.symbol, EXCHANGE_NAME).unwrap();
        let pair_expected = format!(
            "{}/{}",
            &market.buyerCurrencyName, &market.sellerCurrencyName
        );

        assert_eq!(pair.as_str(), pair_expected);

        let market_type = get_market_type(&market.symbol, EXCHANGE_NAME, None);
        assert_eq!(MarketType::LinearSwap, market_type);
    }
}
