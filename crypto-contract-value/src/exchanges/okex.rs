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
                ("ADA/USDT", 100f64),
                ("ALGO/USDT", 10f64),
                ("ALPHA/USDT", 1f64),
                ("ANC/USDT", 1f64),
                ("ANT/USDT", 1f64),
                ("ATOM/USDT", 1f64),
                ("AVAX/USDT", 1f64),
                ("BADGER/USDT", 0.1f64),
                ("BAL/USDT", 0.1f64),
                ("BAND/USDT", 1f64),
                ("BAT/USDT", 10f64),
                ("BCH/USDT", 0.1f64),
                ("BNT/USDT", 10f64),
                ("BSV/USDT", 1f64),
                ("BTC/USDT", 0.01f64),
                ("BTM/USDT", 100f64),
                ("BTT/USDT", 10000f64),
                ("CFX/USDT", 10f64),
                ("CHZ/USDT", 10f64),
                ("COMP/USDT", 0.1f64),
                ("CONV/USDT", 10f64),
                ("CRO/USDT", 10f64),
                ("CRV/USDT", 1f64),
                ("CSPR/USDT", 1f64),
                ("CVC/USDT", 100f64),
                ("DASH/USDT", 0.1f64),
                ("DOGE/USDT", 1000f64),
                ("DORA/USDT", 0.1f64),
                ("DOT/USDT", 1f64),
                ("EGLD/USDT", 0.1f64),
                ("ENJ/USDT", 1f64),
                ("EOS/USDT", 10f64),
                ("ETC/USDT", 10f64),
                ("ETH/USDT", 0.1f64),
                ("FIL/USDT", 0.1f64),
                ("FLM/USDT", 10f64),
                ("FTM/USDT", 10f64),
                ("GRT/USDT", 10f64),
                ("ICP/USDT", 0.01f64),
                ("IOST/USDT", 1000f64),
                ("IOTA/USDT", 10f64),
                ("JST/USDT", 100f64),
                ("KNC/USDT", 1f64),
                ("KSM/USDT", 0.1f64),
                ("LAT/USDT", 10f64),
                ("LINK/USDT", 1f64),
                ("LON/USDT", 1f64),
                ("LPT/USDT", 0.1f64),
                ("LRC/USDT", 10f64),
                ("LTC/USDT", 1f64),
                ("LUNA/USDT", 0.1f64),
                ("MANA/USDT", 10f64),
                ("MASK/USDT", 1f64),
                ("MATIC/USDT", 10f64),
                ("MIR/USDT", 1f64),
                ("MKR/USDT", 0.01f64),
                ("NEAR/USDT", 10f64),
                ("NEO/USDT", 1f64),
                ("OMG/USDT", 1f64),
                ("ONT/USDT", 10f64),
                ("PERP/USDT", 1f64),
                ("QTUM/USDT", 1f64),
                ("REN/USDT", 10f64),
                ("RSR/USDT", 100f64),
                ("RVN/USDT", 10f64),
                ("SAND/USDT", 10f64),
                ("SC/USDT", 100f64),
                ("SHIB/USDT", 1000000f64),
                ("SNX/USDT", 1f64),
                ("SOL/USDT", 1f64),
                ("SRM/USDT", 1f64),
                ("STORJ/USDT", 10f64),
                ("SUN/USDT", 0.1f64),
                ("SUSHI/USDT", 1f64),
                ("SWRV/USDT", 1f64),
                ("THETA/USDT", 10f64),
                ("TORN/USDT", 0.01f64),
                ("TRB/USDT", 0.1f64),
                ("TRX/USDT", 1000f64),
                ("UMA/USDT", 0.1f64),
                ("UNI/USDT", 1f64),
                ("WAVES/USDT", 1f64),
                ("WNXM/USDT", 0.1f64),
                ("XCH/USDT", 0.01f64),
                ("XEM/USDT", 10f64),
                ("XLM/USDT", 100f64),
                ("XMR/USDT", 0.1f64),
                ("XRP/USDT", 100f64),
                ("XTZ/USDT", 1f64),
                ("YFI/USDT", 0.0001f64),
                ("YFII/USDT", 0.001f64),
                ("ZEC/USDT", 0.1f64),
                ("ZEN/USDT", 1f64),
                ("ZIL/USDT", 100f64),
                ("ZRX/USDT", 10f64),
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

        let linear_future: HashMap<String, f64> = {
            let mut m: HashMap<String, f64> = vec![
                ("ADA/USDT", 100f64),
                ("BCH/USDT", 0.1f64),
                ("BSV/USDT", 1f64),
                ("BTC/USDT", 0.01f64),
                ("DOT/USDT", 1f64),
                ("EOS/USDT", 10f64),
                ("ETC/USDT", 10f64),
                ("ETH/USDT", 0.1f64),
                ("FIL/USDT", 0.1f64),
                ("LINK/USDT", 1f64),
                ("LTC/USDT", 1f64),
                ("TRX/USDT", 1000f64),
                ("XRP/USDT", 100f64),
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
        let option: HashMap<String, f64> = vec![
            ("BTC/USD",0.1),
            ("ETH/USD", 1.0),
            ("EOS/USD", 100.0),
        ]
        .into_iter()
        .map(|x| (x.0.to_string(), x.1 as f64))
        .collect();

        let mut result = HashMap::<MarketType, HashMap<String, f64>>::new();
        result.insert(MarketType::LinearSwap, linear_swap);
        result.insert(MarketType::LinearFuture, linear_future);
        result.insert(MarketType::EuropeanOption, option);
        result
    };
}

// get the contract_val field
// market_type, futures, swap, option
fn fetch_contract_val(market_type: &str) -> BTreeMap<String, f64> {
    #[derive(Serialize, Deserialize)]
    struct Instrument {
        instrument_id: String,
        underlying: String,
        contract_val: String,
        is_inverse: String,
    }
    let mut mapping: BTreeMap<String, f64> = BTreeMap::new();

    let txt = http_get(&format!(
        "https://www.okex.com/api/{}/v3/instruments",
        market_type
    ))
    .unwrap();
    let instruments = serde_json::from_str::<Vec<Instrument>>(&txt).unwrap();

    for instrument in instruments.iter().filter(|x| x.is_inverse == "false") {
        let pair = crypto_pair::normalize_pair(&instrument.instrument_id, "okex").unwrap();
        mapping.insert(pair, instrument.contract_val.parse::<f64>().unwrap());
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
    use super::fetch_contract_val;

    #[test]
    fn linear_swap() {
        let mapping = fetch_contract_val("swap");
        for (pair, contract_value) in &mapping {
            println!("(\"{}\", {}f64),", pair, contract_value);
        }
    }

    #[test]
    fn linear_future() {
        let mapping = fetch_contract_val("futures");
        for (pair, contract_value) in &mapping {
            println!("(\"{}\", {}f64),", pair, contract_value);
        }
    }
}
