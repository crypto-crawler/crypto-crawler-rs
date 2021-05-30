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
                ("ADA/USDT", 100f32),
                ("ALGO/USDT", 10f32),
                ("ALPHA/USDT", 1f32),
                ("ANC/USDT", 1f32),
                ("ANT/USDT", 1f32),
                ("ATOM/USDT", 1f32),
                ("AVAX/USDT", 1f32),
                ("BADGER/USDT", 0.1f32),
                ("BAL/USDT", 0.1f32),
                ("BAND/USDT", 1f32),
                ("BAT/USDT", 10f32),
                ("BCH/USDT", 0.1f32),
                ("BNT/USDT", 10f32),
                ("BSV/USDT", 1f32),
                ("BTC/USDT", 0.01f32),
                ("BTM/USDT", 100f32),
                ("BTT/USDT", 10000f32),
                ("CFX/USDT", 10f32),
                ("CHZ/USDT", 10f32),
                ("COMP/USDT", 0.1f32),
                ("CONV/USDT", 10f32),
                ("CRO/USDT", 10f32),
                ("CRV/USDT", 1f32),
                ("CSPR/USDT", 1f32),
                ("CVC/USDT", 100f32),
                ("DASH/USDT", 0.1f32),
                ("DOGE/USDT", 1000f32),
                ("DORA/USDT", 0.1f32),
                ("DOT/USDT", 1f32),
                ("EGLD/USDT", 0.1f32),
                ("ENJ/USDT", 1f32),
                ("EOS/USDT", 10f32),
                ("ETC/USDT", 10f32),
                ("ETH/USDT", 0.1f32),
                ("FIL/USDT", 0.1f32),
                ("FLM/USDT", 10f32),
                ("FTM/USDT", 10f32),
                ("GRT/USDT", 10f32),
                ("ICP/USDT", 0.01f32),
                ("IOST/USDT", 1000f32),
                ("IOTA/USDT", 10f32),
                ("JST/USDT", 100f32),
                ("KNC/USDT", 1f32),
                ("KSM/USDT", 0.1f32),
                ("LAT/USDT", 10f32),
                ("LINK/USDT", 1f32),
                ("LON/USDT", 1f32),
                ("LPT/USDT", 0.1f32),
                ("LRC/USDT", 10f32),
                ("LTC/USDT", 1f32),
                ("LUNA/USDT", 0.1f32),
                ("MANA/USDT", 10f32),
                ("MASK/USDT", 1f32),
                ("MATIC/USDT", 10f32),
                ("MIR/USDT", 1f32),
                ("MKR/USDT", 0.01f32),
                ("NEAR/USDT", 10f32),
                ("NEO/USDT", 1f32),
                ("OMG/USDT", 1f32),
                ("ONT/USDT", 10f32),
                ("PERP/USDT", 1f32),
                ("QTUM/USDT", 1f32),
                ("REN/USDT", 10f32),
                ("RSR/USDT", 100f32),
                ("RVN/USDT", 10f32),
                ("SAND/USDT", 10f32),
                ("SC/USDT", 100f32),
                ("SHIB/USDT", 1000000f32),
                ("SNX/USDT", 1f32),
                ("SOL/USDT", 1f32),
                ("SRM/USDT", 1f32),
                ("STORJ/USDT", 10f32),
                ("SUN/USDT", 0.1f32),
                ("SUSHI/USDT", 1f32),
                ("SWRV/USDT", 1f32),
                ("THETA/USDT", 10f32),
                ("TORN/USDT", 0.01f32),
                ("TRB/USDT", 0.1f32),
                ("TRX/USDT", 1000f32),
                ("UMA/USDT", 0.1f32),
                ("UNI/USDT", 1f32),
                ("WAVES/USDT", 1f32),
                ("WNXM/USDT", 0.1f32),
                ("XCH/USDT", 0.01f32),
                ("XEM/USDT", 10f32),
                ("XLM/USDT", 100f32),
                ("XMR/USDT", 0.1f32),
                ("XRP/USDT", 100f32),
                ("XTZ/USDT", 1f32),
                ("YFI/USDT", 0.0001f32),
                ("YFII/USDT", 0.001f32),
                ("ZEC/USDT", 0.1f32),
                ("ZEN/USDT", 1f32),
                ("ZIL/USDT", 100f32),
                ("ZRX/USDT", 10f32),
            ]
            .into_iter()
            .map(|x| (x.0.to_string(), x.1))
            .collect();

            let from_online = fetch_contract_val("swap");
            for (pair, contract_value) in &from_online {
                m.insert(pair.clone(), *contract_value);
            }

            m
        };

        let linear_future: HashMap<String, f32> = {
            let mut m: HashMap<String, f32> = vec![
                ("ADA/USDT", 100f32),
                ("BCH/USDT", 0.1f32),
                ("BSV/USDT", 1f32),
                ("BTC/USDT", 0.01f32),
                ("DOT/USDT", 1f32),
                ("EOS/USDT", 10f32),
                ("ETC/USDT", 10f32),
                ("ETH/USDT", 0.1f32),
                ("FIL/USDT", 0.1f32),
                ("LINK/USDT", 1f32),
                ("LTC/USDT", 1f32),
                ("TRX/USDT", 1000f32),
                ("XRP/USDT", 100f32),
            ]
            .into_iter()
            .map(|x| (x.0.to_string(), x.1))
            .collect();

            let from_online = fetch_contract_val("futures");
            for (pair, contract_value) in &from_online {
                m.insert(pair.clone(), *contract_value);
            }

            m
        };

        // see https://www.okex.com/docs/en/#option-option---instrument
        let option: HashMap<String, f32> = vec![
            ("BTC/USD",0.1),
            ("ETH/USD", 1.0),
            ("EOS/USD", 100.0),
        ]
        .into_iter()
        .map(|x| (x.0.to_string(), x.1 as f32))
        .collect();

        let mut result = HashMap::<MarketType, HashMap<String, f32>>::new();
        result.insert(MarketType::LinearSwap, linear_swap);
        result.insert(MarketType::LinearFuture, linear_future);
        result.insert(MarketType::EuropeanOption, option);
        result
    };
}

// get the contract_val field
// market_type, futures, swap, option
fn fetch_contract_val(market_type: &str) -> BTreeMap<String, f32> {
    #[derive(Serialize, Deserialize)]
    struct Instrument {
        instrument_id: String,
        underlying: String,
        contract_val: String,
        is_inverse: String,
    }
    let mut mapping: BTreeMap<String, f32> = BTreeMap::new();

    let txt = http_get(&format!(
        "https://www.okex.com/api/{}/v3/instruments",
        market_type
    ))
    .unwrap();
    let instruments = serde_json::from_str::<Vec<Instrument>>(&txt).unwrap();

    for instrument in instruments.iter().filter(|x| x.is_inverse == "false") {
        let pair = crypto_pair::normalize_pair(&instrument.instrument_id, "okex").unwrap();
        mapping.insert(pair, instrument.contract_val.parse::<f32>().unwrap());
    }

    mapping
}

pub(crate) fn get_contract_value(market_type: MarketType, pair: &str) -> Option<f32> {
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
    use super::fetch_contract_val;

    #[test]
    fn linear_swap() {
        let mapping = fetch_contract_val("swap");
        for (pair, contract_value) in &mapping {
            println!("(\"{}\", {}f32),", pair, contract_value);
        }
    }

    #[test]
    fn linear_future() {
        let mapping = fetch_contract_val("futures");
        for (pair, contract_value) in &mapping {
            println!("(\"{}\", {}f32),", pair, contract_value);
        }
    }
}
