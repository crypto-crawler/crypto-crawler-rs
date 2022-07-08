mod utils;

use crypto_market_type::MarketType;
use crypto_pair::{get_market_type, normalize_currency, normalize_pair};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use utils::http_get;

const EXCHANGE_NAME: &str = "kraken";

// https://docs.kraken.com/rest/#operation/getTradableAssetPairs
#[derive(Clone, Serialize, Deserialize)]
struct SpotMarket {
    altname: String,
    wsname: String,
    base: String,
    quote: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct KrakenResponse {
    result: HashMap<String, SpotMarket>,
}

#[test]
fn verify_spot_symbols() {
    let txt = http_get("https://api.kraken.com/0/public/AssetPairs").unwrap();
    let resp = serde_json::from_str::<KrakenResponse>(&txt).unwrap();

    for (_key, market) in resp.result.iter() {
        let pair = normalize_pair(&market.wsname, EXCHANGE_NAME).unwrap();
        let pair2 = normalize_pair(&market.altname, EXCHANGE_NAME).unwrap();
        // let pair3 = normalize_pair(key, EXCHANGE_NAME).unwrap();
        let pair_expected = format!(
            "{}/{}",
            normalize_currency(&market.base, EXCHANGE_NAME),
            normalize_currency(&market.quote, EXCHANGE_NAME)
        );

        assert_eq!(pair.as_str(), pair_expected);
        assert_eq!(pair2.as_str(), pair_expected);
        // assert_eq!(pair3.as_str(), pair_expected);
        assert_eq!(
            MarketType::Spot,
            get_market_type(&market.wsname, EXCHANGE_NAME, None)
        );
        assert_eq!(
            MarketType::Spot,
            get_market_type(&market.altname, EXCHANGE_NAME, None)
        );
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct FuturesMarket {
    symbol: String,
    #[serde(rename = "type")]
    type_: String,
    tradeable: bool,
    underlying: Option<String>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct FuturesResponse<T: Sized> {
    result: String,
    instruments: Vec<T>,
}

// see <https://www.kraken.com/features/api#get-tradable-pairs>
fn fetch_futures_markets_raw() -> Vec<FuturesMarket> {
    let txt = http_get("https://futures.kraken.com/derivatives/api/v3/instruments").unwrap();
    let obj = serde_json::from_str::<FuturesResponse<FuturesMarket>>(&txt).unwrap();
    
    obj
        .instruments
        .into_iter()
        .filter(|x| x.tradeable)
        .filter(|m| m.symbol.starts_with("pi_") || m.symbol.starts_with("fi_"))
        .collect::<Vec<FuturesMarket>>()
}

#[test]
fn verify_futures_symbols() {
    let markets = fetch_futures_markets_raw();

    for market in markets.iter() {
        let underlying = market.underlying.clone().unwrap();
        let pair = normalize_pair(&market.symbol, EXCHANGE_NAME).unwrap();
        let pair_expected = format!(
            "{}/USD",
            normalize_currency(&underlying[3..(underlying.len() - 3)], EXCHANGE_NAME),
        );

        assert_eq!(pair.as_str(), pair_expected);

        let market_type = get_market_type(&market.symbol, EXCHANGE_NAME, None);

        if market.symbol.starts_with("fi_") {
            assert_eq!(market_type, MarketType::InverseFuture);
        } else {
            assert_eq!(market_type, MarketType::InverseSwap);
        }
    }
}
