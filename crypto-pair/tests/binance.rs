mod utils;

use crypto_market_type::MarketType;
use crypto_pair::{get_market_type, normalize_currency, normalize_pair};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use utils::http_get;

const EXCHANGE_NAME: &'static str = "binance";
#[derive(Serialize, Deserialize)]
struct BinanceResponse<T: Sized> {
    symbols: Vec<T>,
}
// Spot, Future and Swap markets
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Market {
    symbol: String,
    baseAsset: String,
    quoteAsset: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct OptionMarket {
    symbol: String,
    quoteAsset: String,
    underlying: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// see <https://binance-docs.github.io/apidocs/spot/en/#exchange-information>
fn fetch_spot_markets_raw() -> Vec<Market> {
    let txt = http_get("https://api.binance.com/api/v3/exchangeInfo").unwrap();
    let resp = serde_json::from_str::<BinanceResponse<Market>>(&txt).unwrap();
    resp.symbols
}

// see <https://binance-docs.github.io/apidocs/delivery/en/#exchange-information>
fn fetch_inverse_markets_raw() -> Vec<Market> {
    let txt = http_get("https://dapi.binance.com/dapi/v1/exchangeInfo").unwrap();
    let resp = serde_json::from_str::<BinanceResponse<Market>>(&txt).unwrap();
    resp.symbols
}

// see <https://binance-docs.github.io/apidocs/futures/en/#exchange-information>
fn fetch_linear_markets_raw() -> Vec<Market> {
    let txt = http_get("https://fapi.binance.com/fapi/v1/exchangeInfo").unwrap();
    let resp = serde_json::from_str::<BinanceResponse<Market>>(&txt).unwrap();
    resp.symbols
}

fn fetch_option_markets_raw() -> Vec<OptionMarket> {
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct OptionData {
        timezone: String,
        serverTime: i64,
        optionContracts: Vec<Value>,
        optionAssets: Vec<Value>,
        optionSymbols: Vec<OptionMarket>,
    }
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct BinanceOptionResponse {
        code: i64,
        msg: String,
        data: OptionData,
    }

    let txt =
        http_get("https://voptions.binance.com/options-api/v1/public/exchange/symbols").unwrap();
    let resp = serde_json::from_str::<BinanceOptionResponse>(&txt).unwrap();
    resp.data.optionSymbols
}

#[test]
fn verify_spot_symbols() {
    let markets = fetch_spot_markets_raw();
    for market in markets.iter() {
        let pair = normalize_pair(&market.symbol, EXCHANGE_NAME).unwrap();
        let pair_expected = format!(
            "{}/{}",
            normalize_currency(&market.baseAsset, EXCHANGE_NAME),
            normalize_currency(&market.quoteAsset, EXCHANGE_NAME)
        );

        assert_eq!(pair, pair_expected);
        assert_eq!(
            MarketType::Spot,
            get_market_type(&market.symbol, EXCHANGE_NAME, Some(true))
        );
    }
}

#[test]
fn verify_inverse_symbols() {
    let markets = fetch_inverse_markets_raw();
    for market in markets.iter() {
        let pair = normalize_pair(&market.symbol, EXCHANGE_NAME).unwrap();
        let pair_expected = format!(
            "{}/{}",
            normalize_currency(&market.baseAsset, EXCHANGE_NAME),
            normalize_currency(&market.quoteAsset, EXCHANGE_NAME)
        );

        assert_eq!(pair, pair_expected);

        let market_type = get_market_type(&market.symbol, EXCHANGE_NAME, None);
        assert!(market_type == MarketType::InverseSwap || market_type == MarketType::InverseFuture);
    }
}

#[test]
fn verify_linear_symbols() {
    let markets = fetch_linear_markets_raw();
    for market in markets.iter() {
        let pair = normalize_pair(&market.symbol, EXCHANGE_NAME).unwrap();
        let pair_expected = format!(
            "{}/{}",
            normalize_currency(&market.baseAsset, EXCHANGE_NAME),
            normalize_currency(&market.quoteAsset, EXCHANGE_NAME)
        );

        assert_eq!(pair, pair_expected);

        let market_type = get_market_type(&market.symbol, EXCHANGE_NAME, None);
        assert!(market_type == MarketType::LinearSwap || market_type == MarketType::LinearFuture);
    }
}

#[test]
fn verify_option_symbols() {
    let markets = fetch_option_markets_raw();
    for market in markets.iter() {
        let pair = normalize_pair(&market.symbol, EXCHANGE_NAME).unwrap();

        let base = market
            .underlying
            .strip_suffix(market.quoteAsset.as_str())
            .unwrap();
        let pair_expected = format!(
            "{}/{}",
            normalize_currency(base, EXCHANGE_NAME),
            normalize_currency(&market.quoteAsset, EXCHANGE_NAME)
        );

        assert_eq!(pair, pair_expected);
        assert_eq!(
            MarketType::EuropeanOption,
            get_market_type(&market.symbol, EXCHANGE_NAME, None)
        );
    }
}
