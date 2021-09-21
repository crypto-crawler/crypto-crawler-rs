pub use crypto_market_type::MarketType;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

use super::utils::http_get;

lazy_static! {
    static ref LINEAR_CONTRACT_VALUES: HashMap<String, f64> = {
        // offline data, in case the network is down
        let mut m: HashMap<String, f64> = vec![
            ("1INCH/USDT", 1_f64),
            ("AAVE/USDT", 0.01_f64),
            ("ACH/USDT", 1_f64),
            ("ADA/USDT", 1_f64),
            ("AGLD/USDT", 0.1_f64),
            ("AKRO/USDT", 1_f64),
            ("ALICE/USDT", 0.1_f64),
            ("ALPHA/USDT", 1_f64),
            ("ANKR/USDT", 1_f64),
            ("ATA/USDT", 1_f64),
            ("ATOM/USDT", 0.1_f64),
            ("AVAX/USDT", 0.1_f64),
            ("AXS/USDT", 1_f64),
            ("BAL/USDT", 0.1_f64),
            ("BAND/USDT", 0.1_f64),
            ("BAT/USDT", 1_f64),
            ("BCH/USDT", 0.01_f64),
            ("BNB/USDT", 0.01_f64),
            ("BP/USDT", 0.1_f64),
            ("BSV/USDT", 0.01_f64),
            ("BTC/USDT", 0.0001_f64),
            ("BTM/USDT", 10_f64),
            ("BTS/USDT", 1_f64),
            ("BTT/USDT", 1_f64),
            ("C98/USDT", 1_f64),
            ("CELR/USDT", 1_f64),
            ("CHZ/USDT", 1_f64),
            ("COMP/USDT", 0.01_f64),
            ("COTI/USDT", 1_f64),
            ("CRV/USDT", 0.1_f64),
            ("CSPR/USDT", 1_f64),
            ("CTK/USDT", 1_f64),
            ("CVC/USDT", 1_f64),
            ("DASH/USDT", 0.01_f64),
            ("DENT/USDT", 1_f64),
            ("DGB/USDT", 1_f64),
            ("DOGE/USDT", 100_f64),
            ("DOT/USDT", 0.1_f64),
            ("EGLD/USDT", 0.1_f64),
            ("ENJ/USDT", 1_f64),
            ("EOS/USDT", 0.1_f64),
            ("ETC/USDT", 0.1_f64),
            ("ETH/USDT", 0.01_f64),
            ("FILECOIN/USDT", 0.01_f64),
            ("FLM/USDT", 1_f64),
            ("FLOW/USDT", 0.1_f64),
            ("FTT/USDT", 0.1_f64),
            ("GRT/USDT", 1_f64),
            ("GTC/USDT", 0.1_f64),
            ("HBAR/USDT", 1_f64),
            ("HNT/USDT", 0.1_f64),
            ("HOT/USDT", 1_f64),
            ("HT/USDT", 0.1_f64),
            ("ICP/USDT", 0.01_f64),
            ("ICX/USDT", 1_f64),
            ("IOST/USDT", 100_f64),
            ("IOTA/USDT", 1_f64),
            ("IOTX/USDT", 1_f64),
            ("KAVA/USDT", 0.1_f64),
            ("KNC/USDT", 0.1_f64),
            ("KSM/USDT", 0.01_f64),
            ("LINA/USDT", 1_f64),
            ("LINK/USDT", 0.1_f64),
            ("LIT/USDT", 0.1_f64),
            ("LRC/USDT", 1_f64),
            ("LTC/USDT", 0.01_f64),
            ("LUNA/USDT", 1_f64),
            ("MANA/USDT", 1_f64),
            ("MASK/USDT", 0.1_f64),
            ("MATIC/USDT", 1_f64),
            ("MKR/USDT", 0.01_f64),
            ("MTL/USDT", 1_f64),
            ("NEAR/USDT", 1_f64),
            ("NEO/USDT", 0.1_f64),
            ("NKN/USDT", 1_f64),
            ("O3/USDT", 1_f64),
            ("OCEAN/USDT", 1_f64),
            ("OGN/USDT", 1_f64),
            ("OKB/USDT", 0.1_f64),
            ("OMG/USDT", 0.1_f64),
            ("ONE/USDT", 1_f64),
            ("ONT/USDT", 1_f64),
            ("QTUM/USDT", 0.1_f64),
            ("RACA/USDT", 1000_f64),
            ("RSR/USDT", 1_f64),
            ("SAND/USDT", 1_f64),
            ("SC/USDT", 1_f64),
            ("SDN/USDT", 0.1_f64),
            ("SHIB/USDT", 1000_f64),
            ("SKL/USDT", 1_f64),
            ("SLP/USDT", 1_f64),
            ("SNX/USDT", 0.1_f64),
            ("SOL/USDT", 0.1_f64),
            ("SRM/USDT", 0.1_f64),
            ("STMX/USDT", 1_f64),
            ("STORJ/USDT", 1_f64),
            ("SUSHI/USDT", 0.1_f64),
            ("SXP/USDT", 1_f64),
            ("THETA/USDT", 1_f64),
            ("TLM/USDT", 1_f64),
            ("TORN/USDT", 0.01_f64),
            ("TRX/USDT", 10_f64),
            ("UNI/USDT", 0.1_f64),
            ("VET/USDT", 1_f64),
            ("WAVES/USDT", 0.1_f64),
            ("XEM/USDT", 1_f64),
            ("XLM/USDT", 10_f64),
            ("XMR/USDT", 0.01_f64),
            ("XRP/USDT", 1_f64),
            ("XTZ/USDT", 0.1_f64),
            ("YFI/USDT", 0.0001_f64),
            ("YFII/USDT", 0.0001_f64),
            ("ZEC/USDT", 0.01_f64),
            ("ZEN/USDT", 0.1_f64),
            ("ZIL/USDT", 10_f64),
            ("ZRX/USDT", 1_f64),
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
            println!("(\"{}\", {}_f64),", pair, contract_value);
        }
    }
}
