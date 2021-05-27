pub use crypto_market_type::MarketType;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};

use super::utils::http_get;

lazy_static! {
    static ref LINEAR_CONTRACT_VALUES: HashMap<String, f32> = {
        // offline data, in case the network is down
        let mut m: HashMap<String, f32> = vec![
            ("1INCH/USDT", 1f32),
            ("AAVE/USDT", 0.01f32),
            ("ADA/USDT", 10f32),
            ("ALGO/USDT", 1f32),
            ("ATOM/USDT", 0.1f32),
            ("AVAX/USDT", 0.1f32),
            ("BAND/USDT", 0.1f32),
            ("BAT/USDT", 1f32),
            ("BCH/USDT", 0.01f32),
            ("BNB/USDT", 0.01f32),
            ("BSV/USDT", 0.01f32),
            ("BTC/USDT", 0.001f32),
            ("BTT/USDT", 1000f32),
            ("CHZ/USDT", 1f32),
            ("COMP/USDT", 0.01f32),
            ("CRV/USDT", 1f32),
            ("DASH/USDT", 0.01f32),
            ("DENT/USDT", 100f32),
            ("DGB/USDT", 10f32),
            ("DOGE/USDT", 100f32),
            ("DOT/USDT", 1f32),
            ("ENJ/USDT", 1f32),
            ("EOS/USDT", 1f32),
            ("ETC/USDT", 0.1f32),
            ("ETH/USDT", 0.01f32),
            ("FIL/USDT", 0.1f32),
            ("FTM/USDT", 1f32),
            ("GRT/USDT", 1f32),
            ("ICP/USDT", 0.01f32),
            ("IOST/USDT", 100f32),
            ("KSM/USDT", 0.01f32),
            ("LINK/USDT", 0.1f32),
            ("LTC/USDT", 0.1f32),
            ("LUNA/USDT", 1f32),
            ("MANA/USDT", 1f32),
            ("MATIC/USDT", 10f32),
            ("MIR/USDT", 0.1f32),
            ("MKR/USDT", 0.001f32),
            ("NEO/USDT", 0.1f32),
            ("OCEAN/USDT", 1f32),
            ("ONT/USDT", 1f32),
            ("QTUM/USDT", 0.1f32),
            ("RVN/USDT", 10f32),
            ("SHIB/USDT", 100000f32),
            ("SNX/USDT", 0.1f32),
            ("SOL/USDT", 0.1f32),
            ("SUSHI/USDT", 1f32),
            ("SXP/USDT", 1f32),
            ("THETA/USDT", 0.1f32),
            ("TRX/USDT", 100f32),
            ("UNI/USDT", 1f32),
            ("VET/USDT", 100f32),
            ("WAVES/USDT", 0.1f32),
            ("XEM/USDT", 1f32),
            ("XLM/USDT", 10f32),
            ("XMR/USDT", 0.01f32),
            ("XRP/USDT", 10f32),
            ("XTZ/USDT", 1f32),
            ("YFI/USDT", 0.0001f32),
            ("ZEC/USDT", 0.01f32),
        ]
        .into_iter()
        .map(|x| (x.0.to_string(), x.1))
        .collect();

        let from_online = fetch_linear_multipliers();
        for (pair, contract_value) in &from_online {
            m.insert(pair.clone(), *contract_value);
        }

        m
    };
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SwapMarket {
    symbol: String,
    multiplier: f32,
    isInverse: bool,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ResponseMsg {
    code: String,
    data: Vec<SwapMarket>,
}

// get the multiplier field from linear markets
fn fetch_linear_multipliers() -> BTreeMap<String, f32> {
    let mut mapping: BTreeMap<String, f32> = BTreeMap::new();

    let txt = http_get("https://api-futures.kucoin.com/api/v1/contracts/active").unwrap();
    let resp = serde_json::from_str::<ResponseMsg>(&txt).unwrap();
    for swap_market in resp.data.iter().filter(|x| !x.isInverse) {
        mapping.insert(
            crypto_pair::normalize_pair(&swap_market.symbol, "kucoin").unwrap(),
            swap_market.multiplier,
        );
    }

    mapping
}

pub(crate) fn get_contract_value(market_type: MarketType, pair: &str) -> Option<f32> {
    match market_type {
        MarketType::InverseSwap | MarketType::InverseFuture => Some(1.0),
        MarketType::LinearSwap => Some(LINEAR_CONTRACT_VALUES[pair]),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::fetch_linear_multipliers;

    #[test]
    fn linear() {
        let mapping = fetch_linear_multipliers();
        for (pair, contract_value) in &mapping {
            println!("(\"{}\", {}f32),", pair, contract_value);
        }
    }
}
