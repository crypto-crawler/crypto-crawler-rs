mod utils;

use crypto_market_type::MarketType;
use crypto_pair::{get_market_type, normalize_currency, normalize_pair};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use utils::http_get;

const EXCHANGE_NAME: &'static str = "okex";

#[derive(Serialize, Deserialize)]
struct Market {
    instrument_id: String,
    base_currency: String,
    quote_currency: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// see <https://www.okex.com/docs/en/#spot-currency>
fn fetch_spot_markets_raw() -> Vec<Market> {
    let txt = http_get("https://www.okex.com/api/spot/v3/instruments").unwrap();
    serde_json::from_str::<Vec<Market>>(&txt).unwrap()
}

// see <https://www.okex.com/docs/en/#swap-swap---contract_information>
fn fetch_swap_markets_raw() -> Vec<Market> {
    let txt = http_get("https://www.okex.com/api/swap/v3/instruments").unwrap();
    serde_json::from_str::<Vec<Market>>(&txt).unwrap()
}

// see <https://www.okex.com/docs/en/#futures-contract_information>
fn fetch_future_markets_raw() -> Vec<Market> {
    let txt = http_get("https://www.okex.com/api/futures/v3/instruments").unwrap();
    serde_json::from_str::<Vec<Market>>(&txt).unwrap()
}

#[test]
fn verify_spot_symbols() {
    let markets = fetch_spot_markets_raw();
    for market in markets.iter() {
        let pair = normalize_pair(&market.instrument_id, EXCHANGE_NAME).unwrap();
        let pair_expected = format!(
            "{}/{}",
            normalize_currency(&market.base_currency, EXCHANGE_NAME),
            normalize_currency(&market.quote_currency, EXCHANGE_NAME)
        );

        assert_eq!(pair.as_str(), pair_expected);

        assert_eq!(
            MarketType::Spot,
            get_market_type(&market.instrument_id, EXCHANGE_NAME, None)
        );
    }
}

#[test]
fn verify_swap_symbols() {
    let markets = fetch_swap_markets_raw();
    for market in markets.iter() {
        let pair = normalize_pair(&market.instrument_id, EXCHANGE_NAME).unwrap();
        let pair_expected = format!(
            "{}/{}",
            normalize_currency(&market.base_currency, EXCHANGE_NAME),
            normalize_currency(&market.quote_currency, EXCHANGE_NAME)
        );

        assert_eq!(pair.as_str(), pair_expected);

        let market_type = get_market_type(&market.instrument_id, EXCHANGE_NAME, None);
        assert!(market_type == MarketType::LinearSwap || market_type == MarketType::InverseSwap);
    }
}

#[test]
fn verify_future_symbols() {
    let markets = fetch_future_markets_raw();
    for market in markets.iter() {
        let pair = normalize_pair(&market.instrument_id, EXCHANGE_NAME).unwrap();
        let pair_expected = format!(
            "{}/{}",
            normalize_currency(&market.base_currency, EXCHANGE_NAME),
            normalize_currency(&market.quote_currency, EXCHANGE_NAME)
        );

        assert_eq!(pair.as_str(), pair_expected);

        let market_type = get_market_type(&market.instrument_id, EXCHANGE_NAME, None);
        assert!(
            market_type == MarketType::LinearFuture || market_type == MarketType::InverseFuture
        );
    }
}

// see <https://www.okex.com/docs/en/#option-option---instrument>
#[derive(Serialize, Deserialize)]
struct OptionMarket {
    instrument_id: String,
    underlying: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// see <https://www.okex.com/docs/en/#option-option---instrument>
fn fetch_option_markets_raw() -> Vec<OptionMarket> {
    let txt = http_get("https://www.okex.com/api/option/v3/underlying").unwrap();
    let underlying_indexes = serde_json::from_str::<Vec<String>>(&txt).unwrap();

    let mut markets = Vec::<OptionMarket>::new();
    for index in underlying_indexes.iter() {
        let url = format!("https://www.okex.com/api/option/v3/instruments/{}", index);
        let txt = http_get(url.as_str()).unwrap();
        let mut arr = serde_json::from_str::<Vec<OptionMarket>>(&txt).unwrap();
        markets.append(&mut arr);
    }

    markets
}

#[test]
fn verify_option_symbols() {
    let markets = fetch_option_markets_raw();
    for market in markets.iter() {
        let pair = normalize_pair(&market.instrument_id, EXCHANGE_NAME).unwrap();
        let pair_expected = market.underlying.replace("-", "/");

        assert_eq!(pair.as_str(), pair_expected);
        assert_eq!(
            MarketType::EuropeanOption,
            get_market_type(&market.instrument_id, EXCHANGE_NAME, None)
        );
    }
}
