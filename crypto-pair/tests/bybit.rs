mod utils;

use crypto_market_type::MarketType;
use crypto_pair::{get_market_type, normalize_currency, normalize_pair};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use utils::http_get;

const EXCHANGE_NAME: &'static str = "bybit";

#[derive(Serialize, Deserialize)]
struct BybitMarket {
    name: String,
    base_currency: String,
    quote_currency: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    ret_code: i64,
    ret_msg: String,
    ext_code: String,
    ext_info: String,
    result: Vec<BybitMarket>,
}

// See https://bybit-exchange.github.io/docs/inverse/#t-querysymbol
fn fetch_swap_markets_raw() -> Vec<BybitMarket> {
    let txt = http_get("https://api.bybit.com/v2/public/symbols").unwrap();
    let resp = serde_json::from_str::<Response>(&txt).unwrap();
    assert_eq!(resp.ret_code, 0);
    resp.result
}

#[test]
fn verify_swap_symbols() {
    let markets = fetch_swap_markets_raw();
    for market in markets.iter() {
        let pair = normalize_pair(&market.name, EXCHANGE_NAME).unwrap();
        let pair_expected = format!(
            "{}/{}",
            normalize_currency(&market.base_currency, EXCHANGE_NAME),
            normalize_currency(&market.quote_currency, EXCHANGE_NAME)
        );

        assert_eq!(pair.as_str(), pair_expected);

        let market_type = get_market_type(&market.name, EXCHANGE_NAME, None);
        println!("{}", market_type);
        assert!(
            market_type == MarketType::LinearSwap
                || market_type == MarketType::InverseSwap
                || market_type == MarketType::InverseFuture
        );
    }
}
