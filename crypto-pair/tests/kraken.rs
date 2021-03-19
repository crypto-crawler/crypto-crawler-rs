mod utils;

use crypto_pair::{normalize_currency, normalize_pair};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use utils::http_get;

const EXCHANGE_NAME: &'static str = "kraken";

#[derive(Clone, Serialize, Deserialize)]
struct SpotMarket {
    altname: String,
    wsname: Option<String>,
    base: String,
    quote: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct KrakenResponse {
    result: HashMap<String, SpotMarket>,
}

// see <https://www.kraken.com/features/api#get-tradable-pairs>
fn fetch_spot_markets_raw() -> Vec<SpotMarket> {
    let txt = http_get("https://api.kraken.com/0/public/AssetPairs").unwrap();
    let resp = serde_json::from_str::<KrakenResponse>(&txt).unwrap();
    let markets = resp
        .result
        .values()
        .into_iter()
        .filter(|x| x.wsname.is_some())
        .map(|x| x.clone())
        .collect::<Vec<SpotMarket>>();
    markets
}

#[test]
fn verify_spot_symbols() {
    let markets = fetch_spot_markets_raw();
    for market in markets.iter() {
        let pair = normalize_pair(&market.wsname.clone().unwrap(), EXCHANGE_NAME).unwrap();
        let pair_expected = format!(
            "{}/{}",
            normalize_currency(&market.base, EXCHANGE_NAME),
            normalize_currency(&market.quote, EXCHANGE_NAME)
        );

        assert_eq!(pair.as_str(), pair_expected);
    }
}
