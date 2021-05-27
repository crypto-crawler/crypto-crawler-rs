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
                ("AAVE/USDT", 0.01f32),
                ("ADA/USDT", 10f32),
                ("ALGO/USDT", 10f32),
                ("ALICE/USDT", 0.1f32),
                ("ALPHA/USDT", 1f32),
                ("ALT/USDT", 0.001f32),
                ("AMPL/USDT", 1f32),
                ("ANC/USDT", 1f32),
                ("ANKR/USDT", 10f32),
                ("ANT/USDT", 0.1f32),
                ("AR/USDT", 0.1f32),
                ("ATOM/USDT", 1f32),
                ("AVAX/USDT", 1f32),
                ("BADGER/USDT", 0.1f32),
                ("BAKE/USDT", 0.1f32),
                ("BAND/USDT", 0.1f32),
                ("BAT/USDT", 10f32),
                ("BCH/USDT", 0.01f32),
                ("BCHA/USDT", 0.1f32),
                ("BEAM/USDT", 10f32),
                ("BNB/USDT", 0.001f32),
                ("BNT/USDT", 1f32),
                ("BSV/USDT", 0.01f32),
                ("BTC/USDT", 0.0001f32),
                ("BTM/USDT", 10f32),
                ("BTS/USDT", 100f32),
                ("CAKE/USDT", 0.1f32),
                ("CFX/USDT", 10f32),
                ("CHR/USDT", 10f32),
                ("CHZ/USDT", 100f32),
                ("CKB/USDT", 100f32),
                ("COMP/USDT", 0.01f32),
                ("CONV/USDT", 10f32),
                ("CRU/USDT", 0.01f32),
                ("CRV/USDT", 0.1f32),
                ("CVC/USDT", 10f32),
                ("DASH/USDT", 0.01f32),
                ("DEFI/USDT", 0.001f32),
                ("DEGO/USDT", 0.1f32),
                ("DOGE/USDT", 10f32),
                ("DOT/USDT", 1f32),
                ("EGLD/USDT", 0.1f32),
                ("EOS/USDT", 1f32),
                ("ETC/USDT", 0.1f32),
                ("ETH/USDT", 0.01f32),
                ("EXCH/USDT", 0.001f32),
                ("FIL/USDT", 0.01f32),
                ("FIL6/USDT", 0.1f32),
                ("FLOW/USDT", 0.1f32),
                ("FRONT/USDT", 1f32),
                ("GRIN/USDT", 10f32),
                ("GRT/USDT", 10f32),
                ("HBAR/USDT", 10f32),
                ("HIVE/USDT", 1f32),
                ("HT/USDT", 1f32),
                ("ICP/USDT", 0.001f32),
                ("IRIS/USDT", 10f32),
                ("JST/USDT", 100f32),
                ("KAVA/USDT", 1f32),
                ("KSM/USDT", 0.1f32),
                ("LINA/USDT", 10f32),
                ("LINK/USDT", 1f32),
                ("LON/USDT", 1f32),
                ("LPT/USDT", 0.1f32),
                ("LRC/USDT", 1f32),
                ("LTC/USDT", 0.1f32),
                ("LUNA/USDT", 1f32),
                ("MASK/USDT", 0.1f32),
                ("MATIC/USDT", 10f32),
                ("MKR/USDT", 0.001f32),
                ("MTL/USDT", 0.1f32),
                ("NEAR/USDT", 1f32),
                ("NEST/USDT", 10f32),
                ("NKN/USDT", 1f32),
                ("OGN/USDT", 1f32),
                ("OKB/USDT", 0.1f32),
                ("OMG/USDT", 1f32),
                ("ONT/USDT", 1f32),
                ("OXY/USDT", 1f32),
                ("PEARL/USDT", 0.001f32),
                ("PERP/USDT", 0.1f32),
                ("POLS/USDT", 1f32),
                ("POND/USDT", 10f32),
                ("PRIV/USDT", 0.001f32),
                ("QTUM/USDT", 1f32),
                ("RNDR/USDT", 1f32),
                ("ROSE/USDT", 100f32),
                ("SERO/USDT", 10f32),
                ("SHIB/USDT", 10000f32),
                ("SKL/USDT", 10f32),
                ("SNX/USDT", 0.1f32),
                ("SOL/USDT", 1f32),
                ("SRM/USDT", 1f32),
                ("STORJ/USDT", 1f32),
                ("SUN/USDT", 0.1f32),
                ("SUPER/USDT", 1f32),
                ("SUSHI/USDT", 1f32),
                ("SXP/USDT", 1f32),
                ("TFUEL/USDT", 10f32),
                ("THETA/USDT", 1f32),
                ("TRU/USDT", 10f32),
                ("TRX/USDT", 100f32),
                ("UNI/USDT", 1f32),
                ("VET/USDT", 100f32),
                ("WAVES/USDT", 1f32),
                ("WSB/USDT", 0.001f32),
                ("XAUG/USDT", 0.001f32),
                ("XCH/USDT", 0.001f32),
                ("XEM/USDT", 1f32),
                ("XLM/USDT", 10f32),
                ("XMR/USDT", 0.01f32),
                ("XRP/USDT", 10f32),
                ("XTZ/USDT", 1f32),
                ("XVS/USDT", 0.01f32),
                ("YFI/USDT", 0.0001f32),
                ("YFII/USDT", 0.001f32),
                ("ZEC/USDT", 0.01f32),
                ("ZEN/USDT", 0.1f32),
                ("ZIL/USDT", 10f32),
                ("ZKS/USDT", 1f32),
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

        let linear_future: HashMap<String, f32> = {
            let mut m: HashMap<String, f32> = vec![("BTC/USDT", 0.0001), ("ETH/USDT", 0.01)]
                .into_iter()
                .map(|x| (x.0.to_string(), x.1 as f32))
                .collect();

            let from_online = fetch_quanto_multipliers(LINEAR_FUTURE_URL);
            for (pair, contract_value) in &from_online {
                m.insert(pair.clone(), *contract_value);
            }

            m
        };

        let mut result = HashMap::<MarketType, HashMap<String, f32>>::new();
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
fn fetch_quanto_multipliers(url: &str) -> BTreeMap<String, f32> {
    #[derive(Serialize, Deserialize)]
    struct RawMarket {
        name: String,
        quanto_multiplier: String,
    }

    let mut mapping: BTreeMap<String, f32> = BTreeMap::new();

    let txt = http_get(url).unwrap_or_else(|_| "[]".to_string());
    let markets = serde_json::from_str::<Vec<RawMarket>>(&txt).unwrap();
    for market in markets.iter() {
        mapping.insert(
            crypto_pair::normalize_pair(&market.name, "gate").unwrap(),
            market.quanto_multiplier.parse::<f32>().unwrap(),
        );
    }

    mapping
}

pub(crate) fn get_contract_value(market_type: MarketType, pair: &str) -> Option<f32> {
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
            println!("(\"{}\", {}f32),", pair, contract_value);
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
