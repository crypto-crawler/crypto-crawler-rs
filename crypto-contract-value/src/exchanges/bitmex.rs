use std::collections::{BTreeMap, HashMap};

use super::utils::http_get;
use crypto_market_type::MarketType;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// key = market_type + pair
static CONTRACT_VALUES: Lazy<HashMap<String, i64>> = Lazy::new(|| {
    // offline data, in case the network is down
    let mut m: HashMap<String, i64> = vec![
        ("inverse_future.BTC/USD", 100000000),
        ("inverse_future.ETH/USD", 1000000000),
        ("inverse_swap.BTC/EUR", 100000000),
        ("inverse_swap.BTC/USD", 100000000),
        ("inverse_swap.ETH/USD", 1000000000),
        ("linear_future.ADA/BTC", 1000000),
        ("linear_future.BTC/USDT", 1),
        ("linear_future.ETH/BTC", 1000),
        ("linear_future.ETH/USDT", 10),
        ("linear_future.XRP/BTC", 1000000),
        ("linear_swap.ADA/USDT", 10000),
        ("linear_swap.ALTMEXT/USDT", 100),
        ("linear_swap.APE/USDT", 1000),
        ("linear_swap.APE_/USDT", 1),
        ("linear_swap.AVAX/USDT", 100),
        ("linear_swap.AXS_/USDT", 1),
        ("linear_swap.BCH/USDT", 10),
        ("linear_swap.BNB/USDT", 100),
        ("linear_swap.BTC/USDT", 1),
        ("linear_swap.DEFIMEXT/USDT", 100),
        ("linear_swap.DOGE/USDT", 10000),
        ("linear_swap.DOT/USDT", 1000),
        ("linear_swap.EOS/USDT", 1000),
        ("linear_swap.ETH/USDT", 10),
        ("linear_swap.ETH_/USDT", 1),
        ("linear_swap.FTM/USDT", 10000),
        ("linear_swap.GAL/USDT", 1000),
        ("linear_swap.GMT/USDT", 1000),
        ("linear_swap.LINK/USDT", 1000),
        ("linear_swap.LINK_/USDT", 1),
        ("linear_swap.LTC/USDT", 100),
        ("linear_swap.LUNA/USDT", 1000),
        ("linear_swap.MANA/USDT", 1000),
        ("linear_swap.MATIC/USDT", 10000),
        ("linear_swap.MATIC_/USDT", 1),
        ("linear_swap.METAMEXT/USDT", 100),
        ("linear_swap.NEAR/USDT", 1000),
        ("linear_swap.OP/USDT", 1000),
        ("linear_swap.SAND/USDT", 1000),
        ("linear_swap.SHIB/USDT", 1000000),
        ("linear_swap.SOL/USDT", 100),
        ("linear_swap.TRX/USDT", 100000),
        ("linear_swap.UNI_/USDT", 1),
        ("linear_swap.USDBRL/BTC", 10000),
        ("linear_swap.XBT_/USDT", 1),
        ("linear_swap.XRP/USDT", 10000),
        ("quanto_future.ETH/USD", 100),
        ("quanto_swap.ADA/USD", 10000),
        ("quanto_swap.APE/USD", 1000),
        ("quanto_swap.AVAX/USD", 1000),
        ("quanto_swap.AXS/USD", 100),
        ("quanto_swap.BCH/USD", 100),
        ("quanto_swap.BNB/USD", 100),
        ("quanto_swap.DOGE/USD", 100000),
        ("quanto_swap.DOT/USD", 1000),
        ("quanto_swap.EOS/USD", 10000),
        ("quanto_swap.ETH/USD", 100),
        ("quanto_swap.EUR/CHF", 10000),
        ("quanto_swap.EUR/USD", 10000),
        ("quanto_swap.EUR/USDT", 1000000),
        ("quanto_swap.GAL/USD", 10000),
        ("quanto_swap.GMT/USD", 10000),
        ("quanto_swap.LINK/USD", 1000),
        ("quanto_swap.LTC/USD", 200),
        ("quanto_swap.LUNA/USD", 10000),
        ("quanto_swap.NEAR/USD", 10000),
        ("quanto_swap.NZD/USDT", 1000000),
        ("quanto_swap.OP/USD", 10000),
        ("quanto_swap.SOL/USD", 100),
        ("quanto_swap.TRX/USD", 100000),
        ("quanto_swap.USD/CHF", 10000),
        ("quanto_swap.USD/CNH", 10000),
        ("quanto_swap.USD/INR", 10000),
        ("quanto_swap.USD/SEK", 10000),
        ("quanto_swap.USD/TRY", 10000),
        ("quanto_swap.USDT/CHF", 1000000),
        ("quanto_swap.USDT/CNH", 1000000),
        ("quanto_swap.USDT/MXN", 1000000),
        ("quanto_swap.USDT/SEK", 1000000),
        ("quanto_swap.USDT/TRY", 1000000),
        ("quanto_swap.USDT/ZAR", 1000000),
        ("quanto_swap.XRP/USD", 20000),
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
    multiplier: i64,
    isQuanto: bool,
    isInverse: bool,
    hasLiquidity: bool,
    openInterest: i64,
    volume: i64,
    volume24h: i64,
    turnover: i64,
    turnover24h: i64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

fn fetch_contract_values() -> BTreeMap<String, i64> {
    let mut mapping: BTreeMap<String, i64> = BTreeMap::new();

    if let Ok(text) = http_get("https://www.bitmex.com/api/v1/instrument/active") {
        let instruments: Vec<Instrument> = serde_json::from_str::<Vec<Instrument>>(&text)
            .unwrap()
            .into_iter()
            .filter(|x| x.state == "Open" && x.hasLiquidity && x.volume24h > 0 && x.turnover24h > 0)
            .collect();

        for instrument in instruments.iter() {
            let market_type = crypto_pair::get_market_type(&instrument.symbol, "bitmex", None);
            let pair = crypto_pair::normalize_pair(&instrument.symbol, "bitmex").unwrap();
            mapping.insert(
                market_type.to_string() + "." + pair.as_str(),
                if instrument.isInverse {
                    debug_assert!(instrument.multiplier < 0);
                    instrument.multiplier.abs()
                } else {
                    instrument.multiplier
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
        Some(CONTRACT_VALUES[&key] as f64 * 1e-8)
    } else {
        Some(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::fetch_contract_values;

    #[ignore]
    #[test]
    fn test_fetch_contract_values() {
        let mut mapping = fetch_contract_values();
        for (key, value) in super::CONTRACT_VALUES.iter() {
            if !mapping.contains_key(key) {
                mapping.insert(key.to_string(), *value);
            }
        }
        for (pair, contract_value) in &mapping {
            println!("(\"{pair}\", {contract_value}),");
        }
    }
}
