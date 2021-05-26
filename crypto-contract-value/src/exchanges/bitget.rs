pub use crypto_market_type::MarketType;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

use super::utils::http_get;

lazy_static! {
    static ref LINEAR_SWAP_CONTRACT_VALUES: HashMap<String, f32> = {
        // offline data, in case the network is down
        let mut m: HashMap<String, f32> = vec![
            ("AAVE/USDT", 0.1f32),
            ("ADA/USDT", 100f32),
            ("ALGO/USDT", 10f32),
            ("ATOM/USDT", 1f32),
            ("BCH/USDT", 0.01f32),
            ("BTC/USDT", 0.001f32),
            ("COMP/USDT", 0.01f32),
            ("DOGE/USDT", 10f32),
            ("DOT/USDT", 1f32),
            ("EOS/USDT", 1f32),
            ("ETC/USDT", 1f32),
            ("ETH/USDT", 0.1f32),
            ("FIL/USDT", 0.1f32),
            ("LINK/USDT", 1f32),
            ("LTC/USDT", 0.1f32),
            ("SUSHI/USDT", 1f32),
            ("TRX/USDT", 100f32),
            ("UNI/USDT", 1f32),
            ("XLM/USDT", 10f32),
            ("XRP/USDT", 10f32),
            ("XTZ/USDT", 1f32),
            ("YFI/USDT", 0.0001f32),
            ("ZEC/USDT", 0.1f32),
        ]
        .into_iter()
        .map(|x| (x.0.to_string(), x.1))
        .collect();

        let from_online = fetch_contract_val();
        for (pair, contract_value) in &from_online {
            m.insert(pair.clone(), contract_value.clone());
        }

        m
    };
}

fn fetch_contract_val() -> BTreeMap<String, f32> {
    // See https://bitgetlimited.github.io/apidoc/en/swap/#contract-information
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct SwapMarket {
        symbol: String,
        contract_val: String,
        forwardContractFlag: bool,
    }

    let txt = http_get("https://capi.bitget.com/api/swap/v3/market/contracts").unwrap();
    let swap_markets = serde_json::from_str::<Vec<SwapMarket>>(&txt).unwrap();

    let mut mapping: BTreeMap<String, f32> = BTreeMap::new();
    for swap_market in swap_markets.iter().filter(|x| x.forwardContractFlag) {
        mapping.insert(
            crypto_pair::normalize_pair(&swap_market.symbol, "bitget").unwrap(),
            swap_market.contract_val.parse::<f32>().unwrap(),
        );
    }

    mapping
}

pub(crate) fn get_contract_value(market_type: MarketType, pair: &str) -> Option<f32> {
    match market_type {
        MarketType::InverseSwap => Some(1.0),
        MarketType::LinearSwap => Some(LINEAR_SWAP_CONTRACT_VALUES[pair]),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::fetch_contract_val;

    #[test]
    fn linear_swap() {
        let mapping = fetch_contract_val();
        for (pair, contract_value) in &mapping {
            println!("(\"{}\", {}f32),", pair, contract_value);
        }
    }
}
