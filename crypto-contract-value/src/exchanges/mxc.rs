pub use crypto_market_type::MarketType;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

use super::utils::http_get;

lazy_static! {
    static ref LINEAR_CONTRACT_VALUES: HashMap<String, f32> = {
        // offline data, in case the network is down
        let mut m: HashMap<String, f32> = vec![
            ("1INCH/USDT", 1f32),
            ("AAVE/USDT", 0.01f32),
            ("ADA/USDT", 1f32),
            ("AKRO/USDT", 1f32),
            ("ALICE/USDT", 0.1f32),
            ("ALPHA/USDT", 1f32),
            ("ANKR/USDT", 1f32),
            ("ATOM/USDT", 0.1f32),
            ("AVAX/USDT", 0.1f32),
            ("AXS/USDT", 1f32),
            ("BAL/USDT", 0.1f32),
            ("BAND/USDT", 0.1f32),
            ("BAT/USDT", 1f32),
            ("BCH/USDT", 0.01f32),
            ("BNB/USDT", 0.01f32),
            ("BSV/USDT", 0.01f32),
            ("BTC/USDT", 0.0001f32),
            ("BTM/USDT", 10f32),
            ("BTS/USDT", 1f32),
            ("BTT/USDT", 1f32),
            ("CELR/USDT", 1f32),
            ("CHZ/USDT", 1f32),
            ("COMP/USDT", 0.01f32),
            ("COTI/USDT", 1f32),
            ("CRV/USDT", 0.1f32),
            ("CTK/USDT", 1f32),
            ("CVC/USDT", 1f32),
            ("DASH/USDT", 0.01f32),
            ("DENT/USDT", 1f32),
            ("DGB/USDT", 1f32),
            ("DOGE/USDT", 100f32),
            ("DOT/USDT", 0.1f32),
            ("EGLD/USDT", 0.1f32),
            ("ENJ/USDT", 1f32),
            ("EOS/USDT", 0.1f32),
            ("ETC/USDT", 0.1f32),
            ("ETH/USDT", 0.01f32),
            ("FILECOIN/USDT", 0.01f32),
            ("FLM/USDT", 1f32),
            ("FLOW/USDT", 0.1f32),
            ("FTT/USDT", 0.1f32),
            ("GRT/USDT", 1f32),
            ("HBAR/USDT", 1f32),
            ("HNT/USDT", 0.1f32),
            ("HOT/USDT", 1f32),
            ("HT/USDT", 0.1f32),
            ("ICP/USDT", 0.01f32),
            ("ICX/USDT", 1f32),
            ("IOST/USDT", 100f32),
            ("IOTA/USDT", 1f32),
            ("KAVA/USDT", 0.1f32),
            ("KNC/USDT", 0.1f32),
            ("KSM/USDT", 0.01f32),
            ("LINA/USDT", 1f32),
            ("LINK/USDT", 0.1f32),
            ("LIT/USDT", 0.1f32),
            ("LRC/USDT", 1f32),
            ("LTC/USDT", 0.01f32),
            ("LUNA/USDT", 1f32),
            ("MANA/USDT", 1f32),
            ("MATIC/USDT", 1f32),
            ("MKR/USDT", 0.01f32),
            ("MTL/USDT", 1f32),
            ("NEAR/USDT", 1f32),
            ("NEO/USDT", 0.1f32),
            ("NKN/USDT", 1f32),
            ("OCEAN/USDT", 1f32),
            ("OGN/USDT", 1f32),
            ("OKB/USDT", 0.1f32),
            ("OMG/USDT", 0.1f32),
            ("ONE/USDT", 1f32),
            ("ONT/USDT", 1f32),
            ("QTUM/USDT", 0.1f32),
            ("RSR/USDT", 1f32),
            ("SAND/USDT", 1f32),
            ("SC/USDT", 1f32),
            ("SHIB/USDT", 1000f32),
            ("SKL/USDT", 1f32),
            ("SNX/USDT", 0.1f32),
            ("SOL/USDT", 0.1f32),
            ("SRM/USDT", 0.1f32),
            ("STMX/USDT", 1f32),
            ("STORJ/USDT", 1f32),
            ("SUSHI/USDT", 0.1f32),
            ("SXP/USDT", 1f32),
            ("THETA/USDT", 1f32),
            ("TRX/USDT", 10f32),
            ("UNI/USDT", 0.1f32),
            ("VET/USDT", 1f32),
            ("WAVES/USDT", 0.1f32),
            ("XEM/USDT", 1f32),
            ("XLM/USDT", 10f32),
            ("XMR/USDT", 0.01f32),
            ("XRP/USDT", 1f32),
            ("XTZ/USDT", 0.1f32),
            ("YFI/USDT", 0.0001f32),
            ("YFII/USDT", 0.0001f32),
            ("ZEC/USDT", 0.01f32),
            ("ZEN/USDT", 0.1f32),
            ("ZIL/USDT", 10f32),
            ("ZRX/USDT", 1f32),
        ]
        .into_iter()
        .map(|x| (x.0.to_string(), x.1))
        .collect();

        let from_online = fetch_linear_contract_sizes();
        for (pair, contract_value) in &from_online {
            m.insert(pair.clone(), *contract_value);
        }

        m
    };
}

// get the contractSize field from linear markets
fn fetch_linear_contract_sizes() -> BTreeMap<String, f32> {
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct SwapMarket {
        symbol: String,
        baseCoin: String,
        quoteCoin: String,
        settleCoin: String,
        contractSize: f32,
    }

    #[derive(Serialize, Deserialize)]
    struct ResponseMsg {
        success: bool,
        code: i64,
        data: Vec<SwapMarket>,
    }

    let mut mapping: BTreeMap<String, f32> = BTreeMap::new();

    let txt = http_get("https://contract.mxc.com/api/v1/contract/detail").unwrap();
    let resp = serde_json::from_str::<ResponseMsg>(&txt).unwrap();
    for linear_market in resp.data.iter().filter(|x| x.settleCoin == x.quoteCoin) {
        mapping.insert(
            crypto_pair::normalize_pair(&linear_market.symbol, "mxc").unwrap(),
            linear_market.contractSize,
        );
    }

    mapping
}

pub(crate) fn get_contract_value(market_type: MarketType, pair: &str) -> Option<f32> {
    match market_type {
        MarketType::InverseSwap => Some(if pair.starts_with("BTC") { 100.0 } else { 10.0 }),
        MarketType::LinearSwap => Some(LINEAR_CONTRACT_VALUES[pair]),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::fetch_linear_contract_sizes;

    #[test]
    fn linear() {
        let mapping = fetch_linear_contract_sizes();
        for (pair, contract_value) in &mapping {
            println!("(\"{}\", {}f32),", pair, contract_value);
        }
    }
}
