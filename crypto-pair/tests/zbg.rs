mod utils;

use crypto_pair::{normalize_currency, normalize_pair};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use utils::http_get;

const EXCHANGE_NAME: &'static str = "zbg";

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct SpotMarket {
    symbol: String,
    base_currency: String,
    quote_currency: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct ResMsg {
    message: String,
    method: Option<String>,
    code: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Response<T: Sized> {
    datas: Vec<T>,
    resMsg: ResMsg,
}

// See https://zbgapi.github.io/docs/spot/v1/en/#public-get-all-supported-trading-symbols
fn fetch_spot_markets_raw() -> Vec<SpotMarket> {
    let txt = http_get("https://www.zbg.com/exchange/api/v1/common/symbols").unwrap();
    let resp = serde_json::from_str::<Response<SpotMarket>>(&txt).unwrap();
    resp.datas
}

#[test]
fn verify_spot_symbols() {
    let markets = fetch_spot_markets_raw();
    for market in markets.iter() {
        let pair = normalize_pair(&market.symbol, EXCHANGE_NAME).unwrap();
        let pair_expected = format!(
            "{}/{}",
            normalize_currency(&market.base_currency, EXCHANGE_NAME),
            normalize_currency(&market.quote_currency, EXCHANGE_NAME)
        );

        assert_eq!(pair.as_str(), pair_expected);
    }
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SwapMarket {
    symbol: String,
    currencyName: String,
    commodityName: Option<String>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// See https://zbgapi.github.io/docs/future/v1/en/#public-get-contracts
fn fetch_swap_markets_raw() -> Vec<SwapMarket> {
    let txt = http_get("https://www.zbg.com/exchange/api/v1/future/common/contracts").unwrap();
    let resp = serde_json::from_str::<Response<SwapMarket>>(&txt).unwrap();
    resp.datas
}

#[test]
fn verify_swap_symbols() {
    let markets = fetch_swap_markets_raw();
    for market in markets.iter().filter(|x| x.commodityName.is_some()) {
        let pair = normalize_pair(&market.symbol, EXCHANGE_NAME).unwrap();
        let pair_expected = if market.symbol.ends_with("_USD-R") {
            format!(
                "{}/{}",
                normalize_currency(&market.currencyName, EXCHANGE_NAME),
                normalize_currency(&market.commodityName.clone().unwrap(), EXCHANGE_NAME)
            )
        } else {
            format!(
                "{}/{}",
                normalize_currency(&market.commodityName.clone().unwrap(), EXCHANGE_NAME),
                normalize_currency(&market.currencyName, EXCHANGE_NAME)
            )
        };

        assert_eq!(pair.as_str(), pair_expected);
    }
}
