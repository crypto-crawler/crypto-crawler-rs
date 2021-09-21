pub use crypto_market_type::MarketType;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

use super::utils::http_get;

lazy_static! {
    static ref LINEAR_CONTRACT_VALUES: HashMap<String, f64> = {
        // offline data, in case the network is down
        let mut m: HashMap<String, f64> = vec![
            ("1INCH/USDT", 1f64),
            ("AAVE/USDT", 0.01f64),
            ("ACH/USDT", 1f64),
            ("ADA/USDT", 1f64),
            ("AGLD/USDT", 0.1f64),
            ("AKRO/USDT", 1f64),
            ("ALICE/USDT", 0.1f64),
            ("ALPHA/USDT", 1f64),
            ("ANKR/USDT", 1f64),
            ("ATA/USDT", 1f64),
            ("ATOM/USDT", 0.1f64),
            ("AVAX/USDT", 0.1f64),
            ("AXS/USDT", 1f64),
            ("BAL/USDT", 0.1f64),
            ("BAND/USDT", 0.1f64),
            ("BAT/USDT", 1f64),
            ("BCH/USDT", 0.01f64),
            ("BNB/USDT", 0.01f64),
            ("BP/USDT", 0.1f64),
            ("BSV/USDT", 0.01f64),
            ("BTC/USDT", 0.0001f64),
            ("BTM/USDT", 10f64),
            ("BTS/USDT", 1f64),
            ("BTT/USDT", 1f64),
            ("C98/USDT", 1f64),
            ("CELR/USDT", 1f64),
            ("CHZ/USDT", 1f64),
            ("COMP/USDT", 0.01f64),
            ("COTI/USDT", 1f64),
            ("CRV/USDT", 0.1f64),
            ("CSPR/USDT", 1f64),
            ("CTK/USDT", 1f64),
            ("CVC/USDT", 1f64),
            ("DASH/USDT", 0.01f64),
            ("DENT/USDT", 1f64),
            ("DGB/USDT", 1f64),
            ("DOGE/USDT", 100f64),
            ("DOT/USDT", 0.1f64),
            ("EGLD/USDT", 0.1f64),
            ("ENJ/USDT", 1f64),
            ("EOS/USDT", 0.1f64),
            ("ETC/USDT", 0.1f64),
            ("ETH/USDT", 0.01f64),
            ("FILECOIN/USDT", 0.01f64),
            ("FLM/USDT", 1f64),
            ("FLOW/USDT", 0.1f64),
            ("FTT/USDT", 0.1f64),
            ("GRT/USDT", 1f64),
            ("GTC/USDT", 0.1f64),
            ("HBAR/USDT", 1f64),
            ("HNT/USDT", 0.1f64),
            ("HOT/USDT", 1f64),
            ("HT/USDT", 0.1f64),
            ("ICP/USDT", 0.01f64),
            ("ICX/USDT", 1f64),
            ("IOST/USDT", 100f64),
            ("IOTA/USDT", 1f64),
            ("IOTX/USDT", 1f64),
            ("KAVA/USDT", 0.1f64),
            ("KNC/USDT", 0.1f64),
            ("KSM/USDT", 0.01f64),
            ("LINA/USDT", 1f64),
            ("LINK/USDT", 0.1f64),
            ("LIT/USDT", 0.1f64),
            ("LRC/USDT", 1f64),
            ("LTC/USDT", 0.01f64),
            ("LUNA/USDT", 1f64),
            ("MANA/USDT", 1f64),
            ("MASK/USDT", 0.1f64),
            ("MATIC/USDT", 1f64),
            ("MKR/USDT", 0.01f64),
            ("MTL/USDT", 1f64),
            ("NEAR/USDT", 1f64),
            ("NEO/USDT", 0.1f64),
            ("NKN/USDT", 1f64),
            ("O3/USDT", 1f64),
            ("OCEAN/USDT", 1f64),
            ("OGN/USDT", 1f64),
            ("OKB/USDT", 0.1f64),
            ("OMG/USDT", 0.1f64),
            ("ONE/USDT", 1f64),
            ("ONT/USDT", 1f64),
            ("QTUM/USDT", 0.1f64),
            ("RACA/USDT", 1000f64),
            ("RSR/USDT", 1f64),
            ("SAND/USDT", 1f64),
            ("SC/USDT", 1f64),
            ("SDN/USDT", 0.1f64),
            ("SHIB/USDT", 1000f64),
            ("SKL/USDT", 1f64),
            ("SLP/USDT", 1f64),
            ("SNX/USDT", 0.1f64),
            ("SOL/USDT", 0.1f64),
            ("SRM/USDT", 0.1f64),
            ("STMX/USDT", 1f64),
            ("STORJ/USDT", 1f64),
            ("SUSHI/USDT", 0.1f64),
            ("SXP/USDT", 1f64),
            ("THETA/USDT", 1f64),
            ("TLM/USDT", 1f64),
            ("TORN/USDT", 0.01f64),
            ("TRX/USDT", 10f64),
            ("UNI/USDT", 0.1f64),
            ("VET/USDT", 1f64),
            ("WAVES/USDT", 0.1f64),
            ("XEM/USDT", 1f64),
            ("XLM/USDT", 10f64),
            ("XMR/USDT", 0.01f64),
            ("XRP/USDT", 1f64),
            ("XTZ/USDT", 0.1f64),
            ("YFI/USDT", 0.0001f64),
            ("YFII/USDT", 0.0001f64),
            ("ZEC/USDT", 0.01f64),
            ("ZEN/USDT", 0.1f64),
            ("ZIL/USDT", 10f64),
            ("ZRX/USDT", 1f64),
        ]
        .into_iter()
        .map(|x| (x.0.to_string(), x.1))
        .collect();

        let from_online = fetch_linear_contract_sizes();
        for (pair, contract_value) in from_online {
            m.insert(pair, contract_value);
        }

        m
    };
}

// get the contractSize field from linear markets
fn fetch_linear_contract_sizes() -> BTreeMap<String, f64> {
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct SwapMarket {
        symbol: String,
        baseCoin: String,
        quoteCoin: String,
        settleCoin: String,
        contractSize: f64,
    }

    #[derive(Serialize, Deserialize)]
    struct ResponseMsg {
        success: bool,
        code: i64,
        data: Vec<SwapMarket>,
    }

    let mut mapping: BTreeMap<String, f64> = BTreeMap::new();

    if let Ok(txt) = http_get("https://contract.mxc.com/api/v1/contract/detail") {
        if let Ok(resp) = serde_json::from_str::<ResponseMsg>(&txt) {
            for linear_market in resp.data.iter().filter(|x| x.settleCoin == x.quoteCoin) {
                mapping.insert(
                    crypto_pair::normalize_pair(&linear_market.symbol, "mxc").unwrap(),
                    linear_market.contractSize,
                );
            }
        }
    }

    mapping
}

pub(crate) fn get_contract_value(market_type: MarketType, pair: &str) -> Option<f64> {
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
            println!("(\"{}\", {}f64),", pair, contract_value);
        }
    }
}
