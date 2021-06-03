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
                ("AAVE/USDT", 0.01f64),
                ("ADA/USDT", 10f64),
                ("ALGO/USDT", 10f64),
                ("ALICE/USDT", 0.1f64),
                ("ALPHA/USDT", 1f64),
                ("ALT/USDT", 0.001f64),
                ("AMPL/USDT", 1f64),
                ("ANC/USDT", 1f64),
                ("ANKR/USDT", 10f64),
                ("ANT/USDT", 0.1f64),
                ("AR/USDT", 0.1f64),
                ("ATOM/USDT", 1f64),
                ("AVAX/USDT", 1f64),
                ("BADGER/USDT", 0.1f64),
                ("BAKE/USDT", 0.1f64),
                ("BAND/USDT", 0.1f64),
                ("BAT/USDT", 10f64),
                ("BCH/USDT", 0.01f64),
                ("BCHA/USDT", 0.1f64),
                ("BEAM/USDT", 10f64),
                ("BNB/USDT", 0.001f64),
                ("BNT/USDT", 1f64),
                ("BSV/USDT", 0.01f64),
                ("BTC/USDT", 0.0001f64),
                ("BTM/USDT", 10f64),
                ("BTS/USDT", 100f64),
                ("CAKE/USDT", 0.1f64),
                ("CFX/USDT", 10f64),
                ("CHR/USDT", 10f64),
                ("CHZ/USDT", 100f64),
                ("CKB/USDT", 100f64),
                ("COMP/USDT", 0.01f64),
                ("CONV/USDT", 10f64),
                ("CRU/USDT", 0.01f64),
                ("CRV/USDT", 0.1f64),
                ("CVC/USDT", 10f64),
                ("DASH/USDT", 0.01f64),
                ("DEFI/USDT", 0.001f64),
                ("DEGO/USDT", 0.1f64),
                ("DOGE/USDT", 10f64),
                ("DOT/USDT", 1f64),
                ("EGLD/USDT", 0.1f64),
                ("EOS/USDT", 1f64),
                ("ETC/USDT", 0.1f64),
                ("ETH/USDT", 0.01f64),
                ("EXCH/USDT", 0.001f64),
                ("FIL/USDT", 0.01f64),
                ("FIL6/USDT", 0.1f64),
                ("FLOW/USDT", 0.1f64),
                ("FRONT/USDT", 1f64),
                ("GRIN/USDT", 10f64),
                ("GRT/USDT", 10f64),
                ("HBAR/USDT", 10f64),
                ("HIVE/USDT", 1f64),
                ("HT/USDT", 1f64),
                ("ICP/USDT", 0.001f64),
                ("IRIS/USDT", 10f64),
                ("JST/USDT", 100f64),
                ("KAVA/USDT", 1f64),
                ("KSM/USDT", 0.1f64),
                ("LINA/USDT", 10f64),
                ("LINK/USDT", 1f64),
                ("LON/USDT", 1f64),
                ("LPT/USDT", 0.1f64),
                ("LRC/USDT", 1f64),
                ("LTC/USDT", 0.1f64),
                ("LUNA/USDT", 1f64),
                ("MASK/USDT", 0.1f64),
                ("MATIC/USDT", 10f64),
                ("MKR/USDT", 0.001f64),
                ("MTL/USDT", 0.1f64),
                ("NEAR/USDT", 1f64),
                ("NEST/USDT", 10f64),
                ("NKN/USDT", 1f64),
                ("OGN/USDT", 1f64),
                ("OKB/USDT", 0.1f64),
                ("OMG/USDT", 1f64),
                ("ONT/USDT", 1f64),
                ("OXY/USDT", 1f64),
                ("PEARL/USDT", 0.001f64),
                ("PERP/USDT", 0.1f64),
                ("POLS/USDT", 1f64),
                ("POND/USDT", 10f64),
                ("PRIV/USDT", 0.001f64),
                ("QTUM/USDT", 1f64),
                ("RNDR/USDT", 1f64),
                ("ROSE/USDT", 100f64),
                ("SERO/USDT", 10f64),
                ("SHIB/USDT", 10000f64),
                ("SKL/USDT", 10f64),
                ("SNX/USDT", 0.1f64),
                ("SOL/USDT", 1f64),
                ("SRM/USDT", 1f64),
                ("STORJ/USDT", 1f64),
                ("SUN/USDT", 0.1f64),
                ("SUPER/USDT", 1f64),
                ("SUSHI/USDT", 1f64),
                ("SXP/USDT", 1f64),
                ("TFUEL/USDT", 10f64),
                ("THETA/USDT", 1f64),
                ("TRU/USDT", 10f64),
                ("TRX/USDT", 100f64),
                ("UNI/USDT", 1f64),
                ("VET/USDT", 100f64),
                ("WAVES/USDT", 1f64),
                ("WSB/USDT", 0.001f64),
                ("XAUG/USDT", 0.001f64),
                ("XCH/USDT", 0.001f64),
                ("XEM/USDT", 1f64),
                ("XLM/USDT", 10f64),
                ("XMR/USDT", 0.01f64),
                ("XRP/USDT", 10f64),
                ("XTZ/USDT", 1f64),
                ("XVS/USDT", 0.01f64),
                ("YFI/USDT", 0.0001f64),
                ("YFII/USDT", 0.001f64),
                ("ZEC/USDT", 0.01f64),
                ("ZEN/USDT", 0.1f64),
                ("ZIL/USDT", 10f64),
                ("ZKS/USDT", 1f64),
            ]
            .into_iter()
            .map(|x| (x.0.to_string(), x.1))
            .collect();

            let from_online = fetch_quanto_multipliers(LINEAR_SWAP_URL);
            for (pair, contract_value) in &from_online {
                m.insert(pair.clone(), *contract_value);
            }

            m
        };

        let linear_future: HashMap<String, f64> = {
            let mut m: HashMap<String, f64> = vec![("BTC/USDT", 0.0001), ("ETH/USDT", 0.01)]
                .into_iter()
                .map(|x| (x.0.to_string(), x.1 as f64))
                .collect();

            let from_online = fetch_quanto_multipliers(LINEAR_FUTURE_URL);
            for (pair, contract_value) in &from_online {
                m.insert(pair.clone(), *contract_value);
            }

            m
        };

        let mut result = HashMap::<MarketType, HashMap<String, f64>>::new();
        result.insert(MarketType::LinearSwap, linear_swap);
        result.insert(MarketType::LinearFuture, linear_future);
        result
    };
}

const LINEAR_SWAP_URL: &str = "https://api.gateio.ws/api/v4/futures/usdt/contracts";
const LINEAR_FUTURE_URL: &str = "https://api.gateio.ws/api/v4/delivery/usdt/contracts";

// get the quanto_multiplier field from:
// https://api.gateio.ws/api/v4/futures/usdt/contracts
// https://api.gateio.ws/api/v4/delivery/usdt/contracts
fn fetch_quanto_multipliers(url: &str) -> BTreeMap<String, f64> {
    #[derive(Serialize, Deserialize)]
    struct RawMarket {
        name: String,
        quanto_multiplier: String,
    }

    let mut mapping: BTreeMap<String, f64> = BTreeMap::new();

    let txt = http_get(url).unwrap_or_else(|_| "[]".to_string());
    let markets = serde_json::from_str::<Vec<RawMarket>>(&txt).unwrap();
    for market in markets.iter() {
        mapping.insert(
            crypto_pair::normalize_pair(&market.name, "gate").unwrap(),
            market.quanto_multiplier.parse::<f64>().unwrap(),
        );
    }

    mapping
}

pub(crate) fn get_contract_value(market_type: MarketType, pair: &str) -> Option<f64> {
    match market_type {
        MarketType::InverseSwap => Some(1.0),
        MarketType::LinearSwap | MarketType::LinearFuture => {
            Some(CONTRACT_VALUES[&market_type][pair])
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{fetch_quanto_multipliers, LINEAR_FUTURE_URL, LINEAR_SWAP_URL};

    #[test]
    fn linear_swap() {
        let mapping = fetch_quanto_multipliers(LINEAR_SWAP_URL);
        for (pair, contract_value) in &mapping {
            println!("(\"{}\", {}f64),", pair, contract_value);
        }
    }

    #[test]
    fn linear_future() {
        let mapping = fetch_quanto_multipliers(LINEAR_FUTURE_URL);
        for (pair, contract_value) in &mapping {
            println!("(\"{}\", {}),", pair, contract_value);
        }
    }
}
