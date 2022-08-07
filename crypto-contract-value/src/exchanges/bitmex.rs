use std::collections::{BTreeMap, HashMap};

use super::utils::http_get;
use crypto_market_type::MarketType;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// key = market_type + pair
static CONTRACT_VALUES: Lazy<HashMap<String, f64>> = Lazy::new(|| {
    // offline data, in case the network is down
    let mut m: HashMap<String, f64> = vec![
        ("inverse_future.BTC/USD", 1.0),
        ("inverse_swap.BTC/EUR", 1.0),
        ("inverse_swap.BTC/USD", 1.0),
        ("inverse_swap.ETH/USD", 1.0),
        ("linear_future.ADA/BTC", 0.01),
        ("linear_future.ETH/BTC", 0.00001),
        ("linear_future.ETH/USDT", 0.00001),
        ("linear_future.XRP/BTC", 0.01),
        ("linear_swap.ADA/USDT", 0.01),
        ("linear_swap.APE/USDT", 0.001),
        ("linear_swap.AVAX/USDT", 0.0001),
        ("linear_swap.BCH/USDT", 0.00001),
        ("linear_swap.BNB/USDT", 0.0001),
        ("linear_swap.BTC/USDT", 0.000001),
        ("linear_swap.DEFIMEXT/USDT", 0.0001),
        ("linear_swap.DOGE/USDT", 0.01),
        ("linear_swap.DOT/USDT", 0.001),
        ("linear_swap.EOS/USDT", 0.001),
        ("linear_swap.ETH/USDT", 0.00001),
        ("linear_swap.FTM/USDT", 0.01),
        ("linear_swap.GAL/USDT", 0.001),
        ("linear_swap.GMT/USDT", 0.001),
        ("linear_swap.LINK/USDT", 0.001),
        ("linear_swap.LTC/USDT", 0.0001),
        ("linear_swap.LUNA/USDT", 0.001),
        ("linear_swap.MANA/USDT", 0.001),
        ("linear_swap.MATIC/USDT", 0.01),
        ("linear_swap.NEAR/USDT", 0.001),
        ("linear_swap.OP/USDT", 0.001),
        ("linear_swap.SAND/USDT", 0.001),
        ("linear_swap.SHIB/USDT", 1.0),
        ("linear_swap.SOL/USDT", 0.0001),
        ("linear_swap.TRX/USDT", 0.1),
        ("linear_swap.XRP/USDT", 0.01),
    ]
    .into_iter()
    .map(|x| (x.0.to_string(), x.1))
    .collect();

    let from_online = fetch_contract_values();
    for (pair, contract_value) in from_online {
        m.insert(pair, contract_value);
    }

    m
});

#[derive(Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Instrument {
    symbol: String,
    state: String,
    typ: String,
    quoteCurrency: String,
    multiplier: f64,
    isQuanto: bool,
    isInverse: bool,
    hasLiquidity: bool,
    openInterest: i64,
    volume: i64,
    volume24h: i64,
    turnover: i64,
    turnover24h: i64,
    underlyingToSettleMultiplier: Option<f64>,
    underlyingToPositionMultiplier: Option<f64>,
    quoteToSettleMultiplier: Option<f64>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

fn fetch_contract_values() -> BTreeMap<String, f64> {
    let mut mapping: BTreeMap<String, f64> = BTreeMap::new();

    if let Ok(text) = http_get("https://www.bitmex.com/api/v1/instrument/active") {
        let instruments: Vec<Instrument> = serde_json::from_str::<Vec<Instrument>>(&text)
            .unwrap()
            .into_iter()
            .filter(|x| x.state == "Open" && x.hasLiquidity && x.volume24h > 0 && x.turnover24h > 0)
            .collect();

        for instrument in instruments
            .iter()
            .filter(|instrument| !instrument.isQuanto && instrument.typ != "IFXXXP")
        {
            let market_type = crypto_pair::get_market_type(&instrument.symbol, "bitmex", None);
            let pair = crypto_pair::normalize_pair(&instrument.symbol, "bitmex").unwrap();
            mapping.insert(
                market_type.to_string() + "." + pair.as_str(),
                if let Some(x) = instrument.underlyingToSettleMultiplier {
                    instrument.multiplier / x
                } else {
                    instrument.multiplier / instrument.quoteToSettleMultiplier.unwrap()
                },
            );
        }
    }

    mapping
}

pub(crate) fn get_contract_value(market_type: MarketType, pair: &str) -> Option<f64> {
    if market_type == MarketType::Unknown {
        panic!("Must be a specific market type");
    }
    let key = market_type.to_string() + "." + pair;
    if CONTRACT_VALUES.contains_key(key.as_str()) {
        Some(CONTRACT_VALUES[&key])
    } else {
        Some(1.0)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::fetch_contract_values;

    #[ignore]
    #[test]
    fn test_fetch_contract_values() {
        let new_data = fetch_contract_values();

        let mut mapping: BTreeMap<String, f64> = BTreeMap::new();
        for (key, value) in super::CONTRACT_VALUES.iter() {
            mapping.insert(key.to_string(), *value);
        }
        for (key, value) in new_data.iter() {
            mapping.insert(key.to_string(), *value);
        }

        for (pair, contract_value) in &mapping {
            println!("(\"{pair}\", {contract_value}),");
        }
    }
}
