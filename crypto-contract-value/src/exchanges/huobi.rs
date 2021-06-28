pub use crypto_market_type::MarketType;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

use super::utils::http_get;

lazy_static! {
    static ref CONTRACT_VALUES: HashMap<MarketType, HashMap<String, f64>> = {
        let linear_swap: HashMap<String, f64> = {
            // offline data, in case the network is down
            let mut m: HashMap<String, f64> = vec![
                ("1INCH/USDT", 1f64),
                ("AAVE/USDT", 0.1f64),
                ("ADA/USDT", 10f64),
                ("AKRO/USDT", 100f64),
                ("ALGO/USDT", 10f64),
                ("ANKR/USDT", 10f64),
                ("ATOM/USDT", 1f64),
                ("AVAX/USDT", 1f64),
                ("BAGS/USDT", 0.01f64),
                ("BAL/USDT", 0.1f64),
                ("BAND/USDT", 1f64),
                ("BAT/USDT", 10f64),
                ("BCH/USDT", 0.01f64),
                ("BLZ/USDT", 10f64),
                ("BNB/USDT", 0.1f64),
                ("BNT/USDT", 1f64),
                ("BSV/USDT", 0.01f64),
                ("BTC/USDT", 0.001f64),
                ("BTS/USDT", 100f64),
                ("BTT/USDT", 1000f64),
                ("CHR/USDT", 10f64),
                ("CHZ/USDT", 10f64),
                ("COMP/USDT", 0.01f64),
                ("CRO/USDT", 10f64),
                ("CRV/USDT", 1f64),
                ("CSPR/USDT", 1f64),
                ("CVC/USDT", 10f64),
                ("DASH/USDT", 0.1f64),
                ("DOGE/USDT", 100f64),
                ("DOT/USDT", 1f64),
                ("ENJ/USDT", 1f64),
                ("EOS/USDT", 1f64),
                ("ETC/USDT", 1f64),
                ("ETH/USDT", 0.01f64),
                ("FIL/USDT", 0.1f64),
                ("FORTH/USDT", 0.1f64),
                ("FRONT/USDT", 1f64),
                ("GRT/USDT", 10f64),
                ("HBAR/USDT", 10f64),
                ("ICP/USDT", 0.01f64),
                ("IOST/USDT", 100f64),
                ("IOTA/USDT", 10f64),
                ("KAVA/USDT", 1f64),
                ("KNC/USDT", 1f64),
                ("KSM/USDT", 0.1f64),
                ("LAT/USDT", 10f64),
                ("LINA/USDT", 10f64),
                ("LINK/USDT", 0.1f64),
                ("LRC/USDT", 10f64),
                ("LTC/USDT", 0.1f64),
                ("LUNA/USDT", 1f64),
                ("MANA/USDT", 10f64),
                ("MASK/USDT", 0.1f64),
                ("MASS/USDT", 1f64),
                ("MATIC/USDT", 100f64),
                ("MDX/USDT", 1f64),
                ("MKR/USDT", 0.001f64),
                ("NEAR/USDT", 1f64),
                ("NEO/USDT", 0.1f64),
                ("O3/USDT", 1f64),
                ("OGN/USDT", 1f64),
                ("OMG/USDT", 1f64),
                ("ONE/USDT", 100f64),
                ("ONT/USDT", 10f64),
                ("PHA/USDT", 1f64),
                ("QTUM/USDT", 1f64),
                ("REEF/USDT", 100f64),
                ("REN/USDT", 10f64),
                ("RNDR/USDT", 1f64),
                ("RSR/USDT", 100f64),
                ("RVN/USDT", 10f64),
                ("SAND/USDT", 10f64),
                ("SHIB/USDT", 1000f64),
                ("SKL/USDT", 10f64),
                ("SNX/USDT", 1f64),
                ("SOL/USDT", 1f64),
                ("STORJ/USDT", 1f64),
                ("SUSHI/USDT", 1f64),
                ("THETA/USDT", 10f64),
                ("TRX/USDT", 100f64),
                ("UMA/USDT", 0.1f64),
                ("UNI/USDT", 1f64),
                ("VET/USDT", 100f64),
                ("WAVES/USDT", 1f64),
                ("WOO/USDT", 10f64),
                ("XCH/USDT", 0.001f64),
                ("XEM/USDT", 10f64),
                ("XLM/USDT", 10f64),
                ("XMR/USDT", 0.01f64),
                ("XRP/USDT", 10f64),
                ("XTZ/USDT", 1f64),
                ("YFI/USDT", 0.0001f64),
                ("YFII/USDT", 0.001f64),
                ("ZEC/USDT", 0.1f64),
                ("ZEN/USDT", 0.1f64),
                ("ZIL/USDT", 100f64),
                ("ZKS/USDT", 0.1f64),
            ]
            .into_iter()
            .map(|x| (x.0.to_string(), x.1))
            .collect();

            let from_online = fetch_contract_size(LINEAR_SWAP_URL);
            for (pair, contract_value) in &from_online {
                m.insert(pair.clone(), *contract_value);
            }

            m
        };

        let linear_option: HashMap<String, f64> = {
            let m: HashMap<String, f64> = vec![("BTC/USDT", 0.001),
            ("ETH/USDT", 0.01),]
                .into_iter()
                .map(|x| (x.0.to_string(), x.1 as f64))
                .collect();

            debug_assert!(!LINEAR_OPTION_URL.is_empty());
            // let from_online = fetch_contract_size(LINEAR_OPTION_URL);
            // for (pair, contract_value) in &from_online {
            //     m.insert(pair.clone(), *contract_value);
            // }

            m
        };

        let mut result = HashMap::<MarketType, HashMap<String, f64>>::new();
        result.insert(MarketType::LinearSwap, linear_swap);
        result.insert(MarketType::EuropeanOption, linear_option);
        result
    };
}

const LINEAR_SWAP_URL: &str = "https://api.hbdm.com/linear-swap-api/v1/swap_contract_info";
const LINEAR_OPTION_URL: &str = "https://api.hbdm.com/option-api/v1/option_contract_info";

// get the contract_size field.
fn fetch_contract_size(url: &str) -> BTreeMap<String, f64> {
    #[derive(Serialize, Deserialize)]
    struct RawMarket {
        symbol: String,
        contract_code: String,
        contract_size: f64,
    }

    #[derive(Serialize, Deserialize)]
    struct Response {
        status: String,
        data: Vec<RawMarket>,
        ts: i64,
    }

    let mut mapping: BTreeMap<String, f64> = BTreeMap::new();

    let txt = http_get(url).unwrap_or_else(|_| "[]".to_string());
    let response = serde_json::from_str::<Response>(&txt).unwrap();
    for market in response.data.iter() {
        mapping.insert(
            crypto_pair::normalize_pair(&market.contract_code, "huobi").unwrap(),
            market.contract_size,
        );
    }

    mapping
}

pub(crate) fn get_contract_value(market_type: MarketType, pair: &str) -> Option<f64> {
    match market_type {
        MarketType::InverseSwap | MarketType::InverseFuture => {
            Some(if pair.starts_with("BTC") { 100.0 } else { 10.0 })
        }
        MarketType::LinearSwap | MarketType::LinearFuture | MarketType::EuropeanOption => {
            Some(CONTRACT_VALUES[&market_type][pair])
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{fetch_contract_size, LINEAR_OPTION_URL, LINEAR_SWAP_URL};

    #[test]
    fn linear_swap() {
        let mapping = fetch_contract_size(LINEAR_SWAP_URL);
        for (pair, contract_value) in &mapping {
            println!("(\"{}\", {}f64),", pair, contract_value);
        }
    }

    #[test]
    #[ignore]
    fn linear_option() {
        let mapping = fetch_contract_size(LINEAR_OPTION_URL);
        for (pair, contract_value) in &mapping {
            println!("(\"{}\", {}),", pair, contract_value);
        }
    }
}
