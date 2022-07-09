mod utils;

use crypto_market_type::MarketType;
use crypto_pair::{get_market_type, normalize_currency, normalize_pair};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use utils::http_get;

const EXCHANGE_NAME: &str = "okx";

// see <https://www.okx.com/docs-v5/en/#rest-api-public-data-get-instruments>
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct RawMarket {
    instType: String,  // Instrument type
    instId: String,    // Instrument ID, e.g. BTC-USD-SWAP
    baseCcy: String,   // Base currency, e.g. BTC inBTC-USDT. Only applicable to SPOT
    quoteCcy: String,  // Quote currency, e.g. USDT in BTC-USDT. Only applicable to SPOT
    settleCcy: String, // Settlement and margin currency, e.g. BTC. Only applicable to FUTURES/SWAP/OPTION
    ctValCcy: String,  // Contract value currency. Only applicable to FUTURES/SWAP/OPTION
    ctType: String,    // Contract type, linear, inverse. Only applicable to FUTURES/SWAP
    state: String,     // Instrument status, live, suspend, preopen, settlement
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// Retrieve a list of instruments.
//
// see <https://www.okx.com/docs-v5/en/#rest-api-public-data-get-instruments>
// instType: SPOT, MARGIN, SWAP, FUTURES, OPTION
fn fetch_raw_markets_raw(inst_type: &str) -> Vec<RawMarket> {
    if inst_type == "OPTION" {
        let underlying_indexes = {
            let txt =
                http_get("https://www.okx.com/api/v5/public/underlying?instType=OPTION").unwrap();
            let json_obj = serde_json::from_str::<HashMap<String, Value>>(&txt).unwrap();
            let data = json_obj.get("data").unwrap().as_array().unwrap()[0]
                .as_array()
                .unwrap();
            data.iter()
                .map(|x| x.as_str().unwrap().to_string())
                .collect::<Vec<String>>()
        };

        let mut markets = Vec::<RawMarket>::new();
        for underlying in underlying_indexes.iter() {
            let url = format!(
                "https://www.okx.com/api/v5/public/instruments?instType=OPTION&uly={}",
                underlying
            );
            let txt = {
                let txt = http_get(url.as_str()).unwrap();
                let json_obj = serde_json::from_str::<HashMap<String, Value>>(&txt).unwrap();
                serde_json::to_string(json_obj.get("data").unwrap()).unwrap()
            };
            let mut arr = serde_json::from_str::<Vec<RawMarket>>(&txt).unwrap();
            markets.append(&mut arr);
        }

        markets
    } else {
        let url = format!(
            "https://www.okx.com/api/v5/public/instruments?instType={}",
            inst_type
        );
        let txt = {
            let txt = http_get(url.as_str()).unwrap();
            let json_obj = serde_json::from_str::<HashMap<String, Value>>(&txt).unwrap();
            serde_json::to_string(json_obj.get("data").unwrap()).unwrap()
        };
        serde_json::from_str::<Vec<RawMarket>>(&txt).unwrap()
    }
}

#[test]
fn verify_spot_symbols() {
    let markets = fetch_raw_markets_raw("SPOT");
    for market in markets.iter() {
        assert_eq!("SPOT", market.instType);
        let pair = normalize_pair(&market.instId, EXCHANGE_NAME).unwrap();
        let pair_expected = format!(
            "{}/{}",
            normalize_currency(&market.baseCcy, EXCHANGE_NAME),
            normalize_currency(&market.quoteCcy, EXCHANGE_NAME)
        );

        assert_eq!(pair.as_str(), pair_expected);

        assert_eq!(
            MarketType::Spot,
            get_market_type(&market.instId, EXCHANGE_NAME, None)
        );
    }
}

#[test]
fn verify_swap_symbols() {
    let markets = fetch_raw_markets_raw("SWAP");
    for market in markets.iter() {
        assert_eq!("SWAP", market.instType);
        let pair = normalize_pair(&market.instId, EXCHANGE_NAME).unwrap();
        let market_type = get_market_type(&market.instId, EXCHANGE_NAME, None);
        if market.ctType == "linear" {
            // linear
            assert_eq!(market.settleCcy, "USDT");
            let pair_expected = format!(
                "{}/{}",
                normalize_currency(&market.ctValCcy, EXCHANGE_NAME),
                normalize_currency(&market.settleCcy, EXCHANGE_NAME)
            );
            assert_eq!(pair.as_str(), pair_expected);
            assert_eq!(MarketType::LinearSwap, market_type);
        } else if market.ctType == "inverse" {
            // linear
            let pair_expected = format!(
                "{}/{}",
                normalize_currency(&market.settleCcy, EXCHANGE_NAME),
                normalize_currency(&market.ctValCcy, EXCHANGE_NAME)
            );
            assert_eq!(pair.as_str(), pair_expected);
            assert_eq!(MarketType::InverseSwap, market_type);
        } else {
            panic!("unknown ctType: {}", market.ctType);
        }
    }
}

#[test]
fn verify_future_symbols() {
    let markets = fetch_raw_markets_raw("FUTURES");
    for market in markets.iter() {
        assert_eq!("FUTURES", market.instType);
        let pair = normalize_pair(&market.instId, EXCHANGE_NAME).unwrap();
        let market_type = get_market_type(&market.instId, EXCHANGE_NAME, None);
        if market.ctType == "linear" {
            // linear
            assert_eq!(market.settleCcy, "USDT");
            let pair_expected = format!(
                "{}/{}",
                normalize_currency(&market.ctValCcy, EXCHANGE_NAME),
                normalize_currency(&market.settleCcy, EXCHANGE_NAME)
            );
            assert_eq!(pair.as_str(), pair_expected);
            assert_eq!(MarketType::LinearFuture, market_type);
        } else if market.ctType == "inverse" {
            // linear
            let pair_expected = format!(
                "{}/{}",
                normalize_currency(&market.settleCcy, EXCHANGE_NAME),
                normalize_currency(&market.ctValCcy, EXCHANGE_NAME)
            );
            assert_eq!(pair.as_str(), pair_expected);
            assert_eq!(MarketType::InverseFuture, market_type);
        } else {
            panic!("unknown ctType: {}", market.ctType);
        }
    }
}

#[test]
fn verify_option_symbols() {
    let markets = fetch_raw_markets_raw("OPTION");
    for market in markets.iter() {
        assert_eq!("OPTION", market.instType);
        let pair = normalize_pair(&market.instId, EXCHANGE_NAME).unwrap();
        let pair_expected = format!(
            "{}/USD",
            normalize_currency(&market.settleCcy, EXCHANGE_NAME)
        );

        assert_eq!(pair.as_str(), pair_expected);

        assert_eq!(
            MarketType::EuropeanOption,
            get_market_type(&market.instId, EXCHANGE_NAME, None)
        );
    }
}
