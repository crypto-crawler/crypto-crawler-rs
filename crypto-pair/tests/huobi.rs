mod utils;

use crypto_market_type::MarketType;
use crypto_pair::{get_market_type, normalize_currency, normalize_pair};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use utils::http_get;

const EXCHANGE_NAME: &str = "huobi";

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct SpotMarket {
    base_currency: String,
    quote_currency: String,
    symbol: String,
    state: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response<T: Sized> {
    status: String,
    data: Vec<T>,
}

// see <https://huobiapi.github.io/docs/spot/v1/en/#get-all-supported-trading-symbol>
fn fetch_spot_markets_raw() -> Vec<SpotMarket> {
    let txt = http_get("https://api.huobi.pro/v1/common/symbols").unwrap();
    let resp = serde_json::from_str::<Response<SpotMarket>>(&txt).unwrap();
    resp.data
}

#[test]
fn verify_spot_symbols() {
    let markets = fetch_spot_markets_raw();
    for market in markets.iter().filter(|x| x.state == "online") {
        let pair = normalize_pair(&market.symbol, EXCHANGE_NAME).unwrap();
        let pair_expected = format!(
            "{}/{}",
            normalize_currency(&market.base_currency, EXCHANGE_NAME),
            normalize_currency(&market.quote_currency, EXCHANGE_NAME)
        );

        assert_eq!(pair.as_str(), pair_expected);
        assert_eq!(
            MarketType::Spot,
            get_market_type(&market.symbol, EXCHANGE_NAME, None)
        );
    }
}

#[test]
fn verify_inverse_future_symbols() {
    assert_eq!(
        "BTC/USD".to_string(),
        normalize_pair("BTC_CW", EXCHANGE_NAME).unwrap()
    );
    assert_eq!(
        "BTC/USD".to_string(),
        normalize_pair("BTC_CQ", EXCHANGE_NAME).unwrap()
    );

    assert_eq!(
        MarketType::InverseFuture,
        get_market_type("BTC_CW", EXCHANGE_NAME, None)
    );
    assert_eq!(
        MarketType::InverseFuture,
        get_market_type("BTC_CQ", EXCHANGE_NAME, None)
    );
}

#[test]
fn verify_swap_symbols() {
    assert_eq!(
        "BTC/USD".to_string(),
        normalize_pair("BTC-USD", EXCHANGE_NAME).unwrap()
    );
    assert_eq!(
        "BTC/USDT".to_string(),
        normalize_pair("BTC-USDT", EXCHANGE_NAME).unwrap()
    );

    assert_eq!(
        MarketType::InverseSwap,
        get_market_type("BTC-USD", EXCHANGE_NAME, None)
    );
    assert_eq!(
        MarketType::LinearSwap,
        get_market_type("BTC-USDT", EXCHANGE_NAME, None)
    );
}

#[derive(Serialize, Deserialize)]
struct OptionMarket {
    symbol: String,
    contract_code: String,
    delivery_asset: String,
    quote_asset: String,
    trade_partition: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// see <https://huobiapi.github.io/docs/option/v1/en/#query-option-info>
fn fetch_option_markets_raw() -> Vec<OptionMarket> {
    let txt = http_get("https://api.hbdm.com/option-api/v1/option_contract_info").unwrap();
    let resp = serde_json::from_str::<Response<OptionMarket>>(&txt).unwrap();
    resp.data
}

#[test]
#[ignore]
fn verify_option_symbols() {
    let markets = fetch_option_markets_raw();
    for market in markets.iter() {
        let pair = normalize_pair(&market.contract_code, EXCHANGE_NAME).unwrap();
        let pair_expected = format!(
            "{}/{}",
            normalize_currency(&market.symbol, EXCHANGE_NAME),
            normalize_currency(&market.quote_asset, EXCHANGE_NAME)
        );

        assert_eq!(pair.as_str(), pair_expected);
    }
}
