pub use crypto_market_type::MarketType;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

use super::utils::http_get;

lazy_static! {
    static ref CONTRACT_VALUES: HashMap<MarketType, HashMap<String, f32>> = {
        let linear_swap: HashMap<String, f32> = {
            // offline data, in case the network is down
            let mut m: HashMap<String, f32> = vec![
                ("1INCH/USDT", 1f32),
                ("AAVE/USDT", 0.1f32),
                ("ADA/USDT", 10f32),
                ("AKRO/USDT", 100f32),
                ("ALGO/USDT", 10f32),
                ("ANKR/USDT", 10f32),
                ("ATOM/USDT", 1f32),
                ("AVAX/USDT", 1f32),
                ("BAGS/USDT", 0.01f32),
                ("BAL/USDT", 0.1f32),
                ("BAND/USDT", 1f32),
                ("BAT/USDT", 10f32),
                ("BCH/USDT", 0.01f32),
                ("BLZ/USDT", 10f32),
                ("BNB/USDT", 0.1f32),
                ("BNT/USDT", 1f32),
                ("BSV/USDT", 0.01f32),
                ("BTC/USDT", 0.001f32),
                ("BTS/USDT", 100f32),
                ("BTT/USDT", 1000f32),
                ("CHR/USDT", 10f32),
                ("CHZ/USDT", 10f32),
                ("COMP/USDT", 0.01f32),
                ("CRO/USDT", 10f32),
                ("CRV/USDT", 1f32),
                ("CSPR/USDT", 1f32),
                ("CVC/USDT", 10f32),
                ("DASH/USDT", 0.1f32),
                ("DOGE/USDT", 100f32),
                ("DOT/USDT", 1f32),
                ("ENJ/USDT", 1f32),
                ("EOS/USDT", 1f32),
                ("ETC/USDT", 1f32),
                ("ETH/USDT", 0.01f32),
                ("FIL/USDT", 0.1f32),
                ("FORTH/USDT", 0.1f32),
                ("FRONT/USDT", 1f32),
                ("GRT/USDT", 10f32),
                ("HBAR/USDT", 10f32),
                ("ICP/USDT", 0.01f32),
                ("IOST/USDT", 100f32),
                ("IOTA/USDT", 10f32),
                ("KAVA/USDT", 1f32),
                ("KNC/USDT", 1f32),
                ("KSM/USDT", 0.1f32),
                ("LAT/USDT", 10f32),
                ("LINA/USDT", 10f32),
                ("LINK/USDT", 0.1f32),
                ("LRC/USDT", 10f32),
                ("LTC/USDT", 0.1f32),
                ("LUNA/USDT", 1f32),
                ("MANA/USDT", 10f32),
                ("MASK/USDT", 0.1f32),
                ("MASS/USDT", 1f32),
                ("MATIC/USDT", 100f32),
                ("MDX/USDT", 1f32),
                ("MKR/USDT", 0.001f32),
                ("NEAR/USDT", 1f32),
                ("NEO/USDT", 0.1f32),
                ("O3/USDT", 1f32),
                ("OGN/USDT", 1f32),
                ("OMG/USDT", 1f32),
                ("ONE/USDT", 100f32),
                ("ONT/USDT", 10f32),
                ("PHA/USDT", 1f32),
                ("QTUM/USDT", 1f32),
                ("REEF/USDT", 100f32),
                ("REN/USDT", 10f32),
                ("RNDR/USDT", 1f32),
                ("RSR/USDT", 100f32),
                ("RVN/USDT", 10f32),
                ("SAND/USDT", 10f32),
                ("SHIB/USDT", 1000f32),
                ("SKL/USDT", 10f32),
                ("SNX/USDT", 1f32),
                ("SOL/USDT", 1f32),
                ("STORJ/USDT", 1f32),
                ("SUSHI/USDT", 1f32),
                ("THETA/USDT", 10f32),
                ("TRX/USDT", 100f32),
                ("UMA/USDT", 0.1f32),
                ("UNI/USDT", 1f32),
                ("VET/USDT", 100f32),
                ("WAVES/USDT", 1f32),
                ("WOO/USDT", 10f32),
                ("XCH/USDT", 0.001f32),
                ("XEM/USDT", 10f32),
                ("XLM/USDT", 10f32),
                ("XMR/USDT", 0.01f32),
                ("XRP/USDT", 10f32),
                ("XTZ/USDT", 1f32),
                ("YFI/USDT", 0.0001f32),
                ("YFII/USDT", 0.001f32),
                ("ZEC/USDT", 0.1f32),
                ("ZEN/USDT", 0.1f32),
                ("ZIL/USDT", 100f32),
                ("ZKS/USDT", 0.1f32),
            ]
            .into_iter()
            .map(|x| (x.0.to_string(), x.1))
            .collect();

            let from_online = fetch_contract_size(LINEAR_SWAP_URL);
            for (pair, contract_value) in &from_online {
                m.insert(pair.clone(), contract_value.clone());
            }

            m
        };

        let linear_option: HashMap<String, f32> = {
            let mut m: HashMap<String, f32> = vec![("BTC/USDT", 0.001),
            ("ETH/USDT", 0.01),]
                .into_iter()
                .map(|x| (x.0.to_string(), x.1 as f32))
                .collect();

            let from_online = fetch_contract_size(LINEAR_OPTION_URL);
            for (pair, contract_value) in &from_online {
                m.insert(pair.clone(), contract_value.clone());
            }

            m
        };

        let mut result = HashMap::<MarketType, HashMap<String, f32>>::new();
        result.insert(MarketType::LinearSwap, linear_swap);
        result.insert(MarketType::Option, linear_option);
        result
    };
}

const LINEAR_SWAP_URL: &str = "https://api.hbdm.com/linear-swap-api/v1/swap_contract_info";
const LINEAR_OPTION_URL: &str = "https://api.hbdm.com/option-api/v1/option_contract_info";

// get the contract_size field.
fn fetch_contract_size(url: &str) -> BTreeMap<String, f32> {
    #[derive(Serialize, Deserialize)]
    struct RawMarket {
        symbol: String,
        contract_code: String,
        contract_size: f32,
    }

    #[derive(Serialize, Deserialize)]
    struct Response {
        status: String,
        data: Vec<RawMarket>,
        ts: i64,
    }

    let mut mapping: BTreeMap<String, f32> = BTreeMap::new();

    let txt = http_get(url).unwrap_or("[]".to_string());
    let response = serde_json::from_str::<Response>(&txt).unwrap();
    for market in response.data.iter() {
        mapping.insert(
            crypto_pair::normalize_pair(&market.contract_code, "huobi").unwrap(),
            market.contract_size,
        );
    }

    mapping
}

pub(crate) fn get_contract_value(market_type: MarketType, pair: &str) -> Option<f32> {
    match market_type {
        MarketType::InverseSwap | MarketType::InverseFuture => {
            Some(if pair.starts_with("BTC") { 100.0 } else { 10.0 })
        }
        MarketType::LinearSwap | MarketType::LinearFuture | MarketType::Option => {
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
            println!("(\"{}\", {}f32),", pair, contract_value);
        }
    }

    #[test]
    fn linear_option() {
        let mapping = fetch_contract_size(LINEAR_OPTION_URL);
        for (pair, contract_value) in &mapping {
            println!("(\"{}\", {}),", pair, contract_value);
        }
    }
}
