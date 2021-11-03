use crypto_market_type::MarketType;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

use super::utils::http_get;

lazy_static! {
    static ref SWAP_CONTRACT_VALUES: HashMap<String, f64> = {
        // offline data, in case the network is down
        let mut m: HashMap<String, f64> = vec![
            ("AXS/USDT", 0.1_f64),
            ("BCH/USDT", 0.1_f64),
            ("BSV/USDT", 0.1_f64),
            ("BTC/USD", 1_f64),
            ("BTC/USDT", 0.01_f64),
            ("BTC/ZUSD", 0.01_f64),
            ("DOGE/USDT", 100_f64),
            ("DOT/USDT", 1_f64),
            ("EOS/USDT", 1_f64),
            ("ETC/USDT", 1_f64),
            ("ETH/USD", 1_f64),
            ("ETH/USDT", 0.1_f64),
            ("FIL/USDT", 0.1_f64),
            ("ICP/USDT", 0.1_f64),
            ("LINK/USDT", 1_f64),
            ("LTC/USDT", 0.1_f64),
            ("RHI/ZUSD", 0.01_f64),
            ("SUSHI/USDT", 1_f64),
            ("UNI/USDT", 0.1_f64),
            ("XRP/USDT", 10_f64),
        ]
        .into_iter()
        .map(|x| (x.0.to_string(), x.1))
        .collect();

        let from_online = fetch_contract_val();
        for (pair, contract_value) in from_online {
            m.insert(pair, contract_value);
        }

        m
    };
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SwapMarket {
    symbol: String,
    contractUnit: String,
}

// See https://zbgapi.github.io/docs/future/v1/en/#public-get-contracts
fn fetch_swap_markets_raw() -> Vec<SwapMarket> {
    #[derive(Serialize, Deserialize)]
    struct ResMsg {
        message: String,
        method: Option<String>,
        code: String,
    }
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct Response {
        datas: Vec<SwapMarket>,
        resMsg: ResMsg,
    }
    if let Ok(txt) = http_get("https://www.zbg.com/exchange/api/v1/future/common/contracts") {
        if let Ok(resp) = serde_json::from_str::<Response>(&txt) {
            if resp.resMsg.code != "1" {
                Vec::new()
            } else {
                resp.datas
            }
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    }
}

fn fetch_contract_val() -> BTreeMap<String, f64> {
    let mut mapping: BTreeMap<String, f64> = BTreeMap::new();
    let markets = fetch_swap_markets_raw();
    for market in markets {
        let contract_value = market.contractUnit.parse::<f64>().unwrap();
        assert!(contract_value > 0.0);
        mapping.insert(
            crypto_pair::normalize_pair(&market.symbol, "zbg").unwrap(),
            contract_value,
        );
    }
    mapping
}

pub(crate) fn get_contract_value(market_type: MarketType, pair: &str) -> Option<f64> {
    match market_type {
        MarketType::InverseSwap | MarketType::LinearSwap => Some(SWAP_CONTRACT_VALUES[pair]),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::fetch_contract_val;

    #[test]
    fn print_contract_values() {
        let mapping = fetch_contract_val();
        for (pair, contract_value) in &mapping {
            println!("(\"{}\", {}_f64),", pair, contract_value);
        }
    }
}
