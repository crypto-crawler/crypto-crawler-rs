use crypto_market_type::MarketType;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

use super::utils::http_get;

static CONTRACT_VALUES: Lazy<HashMap<MarketType, HashMap<String, f64>>> = Lazy::new(|| {
    let linear_swap: HashMap<String, f64> = {
        // offline data, in case the network is down
        let mut m: HashMap<String, f64> = vec![
            ("1INCH/USDT", 1_f64),
            ("AAVE/USDT", 0.1_f64),
            ("ACH/USDT", 10_f64),
            ("ADA/USDT", 10_f64),
            ("AGLD/USDT", 1_f64),
            ("AKRO/USDT", 100_f64),
            ("ALGO/USDT", 10_f64),
            ("ALICE/USDT", 0.1_f64),
            ("ANKR/USDT", 10_f64),
            ("AR/USDT", 0.1_f64),
            ("ARPA/USDT", 10_f64),
            ("ATOM/USDT", 1_f64),
            ("AVAX/USDT", 1_f64),
            ("AXS/USDT", 0.1_f64),
            ("BADGER/USDT", 0.1_f64),
            ("BAGS/USDT", 0.01_f64),
            ("BAL/USDT", 0.1_f64),
            ("BAND/USDT", 1_f64),
            ("BAT/USDT", 10_f64),
            ("BCH/USDT", 0.01_f64),
            ("BLZ/USDT", 10_f64),
            ("BICO/USDT", 0.1_f64),
            ("BNB/USDT", 0.1_f64),
            ("BNT/USDT", 1_f64),
            ("BSV/USDT", 0.01_f64),
            ("BTC/USDT", 0.001_f64),
            ("BTS/USDT", 100_f64),
            ("BTT/USDT", 1000000_f64), // changed at 2022-02-28
            ("C98/USDT", 1_f64),
            ("CELO/USDT", 0.1_f64),
            ("CELR/USDT", 10_f64),
            ("CHR/USDT", 10_f64),
            ("CHZ/USDT", 10_f64),
            ("CLV/USDT", 1_f64),
            ("COMP/USDT", 0.01_f64),
            ("COTI/USDT", 10_f64),
            ("CRO/USDT", 10_f64),
            ("CRV/USDT", 1_f64),
            ("CSPR/USDT", 1_f64),
            ("CTSI/USDT", 10_f64),
            ("CVC/USDT", 10_f64),
            ("DASH/USDT", 0.1_f64),
            ("DOGE/USDT", 100_f64),
            ("DOT/USDT", 1_f64),
            ("DYDX/USDT", 0.1_f64),
            ("EGLD/USDT", 0.01_f64),
            ("ENJ/USDT", 1_f64),
            ("ENS/USDT", 0.01_f64),
            ("EOS/USDT", 1_f64),
            ("ETC/USDT", 1_f64),
            ("ETH/USDT", 0.01_f64),
            ("FIL/USDT", 0.1_f64),
            ("FLOW/USDT", 0.1_f64),
            ("FORTH/USDT", 0.1_f64),
            ("FRONT/USDT", 1_f64),
            ("FTM/USDT", 1_f64),
            ("FTT/USDT", 0.01_f64),
            ("GALA/USDT", 10_f64),
            ("GRT/USDT", 10_f64),
            ("HBAR/USDT", 10_f64),
            ("ICP/USDT", 0.01_f64),
            ("IMX/USDT", 0.1_f64),
            ("IOST/USDT", 100_f64),
            ("IOTA/USDT", 10_f64),
            ("IOTX/USDT", 10_f64),
            ("JST/USDT", 10_f64),
            ("KAVA/USDT", 1_f64),
            ("KNC/USDT", 1_f64),
            ("KSM/USDT", 0.1_f64),
            ("LAT/USDT", 10_f64),
            ("LINA/USDT", 10_f64),
            ("LINK/USDT", 0.1_f64),
            ("LOOKS/USDT", 0.1_f64),
            ("LPT/USDT", 0.01_f64),
            ("LRC/USDT", 10_f64),
            ("LTC/USDT", 0.1_f64),
            ("LUNA/USDT", 1_f64),
            ("MANA/USDT", 10_f64),
            ("MASK/USDT", 0.1_f64),
            ("MASS/USDT", 1_f64),
            ("MATIC/USDT", 100_f64),
            ("MDX/USDT", 1_f64),
            ("MINA/USDT", 0.1_f64),
            ("MIR/USDT", 1_f64),
            ("MKR/USDT", 0.001_f64),
            ("NEAR/USDT", 1_f64),
            ("NEO/USDT", 0.1_f64),
            ("NFT/USDT", 1000000_f64),
            ("NKN/USDT", 10_f64),
            ("O3/USDT", 1_f64),
            ("OGN/USDT", 1_f64),
            ("OKB/USDT", 0.1_f64),
            ("OMG/USDT", 1_f64),
            ("ONE/USDT", 100_f64),
            ("ONT/USDT", 10_f64),
            ("PHA/USDT", 1_f64),
            ("PEOPLE/USDT", 10_f64),
            ("QNT/USDT", 0.01_f64),
            ("PHA/USDT", 1_f64),
            ("QTUM/USDT", 1_f64),
            ("REEF/USDT", 100_f64),
            ("REN/USDT", 10_f64),
            ("RNDR/USDT", 1_f64),
            ("ROSE/USDT", 1_f64),
            ("RSR/USDT", 100_f64),
            ("RVN/USDT", 10_f64),
            ("SAND/USDT", 10_f64),
            ("SHIB/USDT", 1000_f64),
            ("SKL/USDT", 10_f64),
            ("SLP/USDT", 100_f64),
            ("SNX/USDT", 1_f64),
            ("SOL/USDT", 1_f64),
            ("SOS/USDT", 100000_f64),
            ("SRM/USDT", 1_f64),
            ("STORJ/USDT", 1_f64),
            ("SUSHI/USDT", 1_f64),
            ("SXP/USDT", 1_f64),
            ("THETA/USDT", 10_f64),
            ("TRIBE/USDT", 10_f64),
            ("TRX/USDT", 100_f64),
            ("UMA/USDT", 0.1_f64),
            ("UNI/USDT", 1_f64),
            ("VET/USDT", 100_f64),
            ("VRA/USDT", 100_f64),
            ("WAVES/USDT", 1_f64),
            ("WAXP/USDT", 10_f64),
            ("WOO/USDT", 10_f64),
            ("XCH/USDT", 0.001_f64),
            ("XEC/USDT", 10000_f64),
            ("XEM/USDT", 10_f64),
            ("XLM/USDT", 10_f64),
            ("XMR/USDT", 0.01_f64),
            ("XRP/USDT", 10_f64),
            ("XTZ/USDT", 1_f64),
            ("YFI/USDT", 0.0001_f64),
            ("YFII/USDT", 0.001_f64),
            ("YGG/USDT", 1_f64),
            ("ZEC/USDT", 0.1_f64),
            ("ZEN/USDT", 0.1_f64),
            ("ZIL/USDT", 100_f64),
            ("ZKS/USDT", 0.1_f64),
        ]
        .into_iter()
        .map(|x| (x.0.to_string(), x.1))
        .collect();

        let from_online = fetch_contract_size(LINEAR_SWAP_URL);
        for (pair, contract_value) in from_online {
            m.insert(pair, contract_value);
        }

        m
    };

    let linear_option: HashMap<String, f64> = {
        let m: HashMap<String, f64> = vec![("BTC/USDT", 0.001), ("ETH/USDT", 0.01)]
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
});

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

    if let Ok(txt) = http_get(url) {
        if let Ok(response) = serde_json::from_str::<Response>(&txt) {
            for market in response.data.iter() {
                mapping.insert(
                    crypto_pair::normalize_pair(&market.contract_code, "huobi").unwrap(),
                    market.contract_size,
                );
            }
        }
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
            println!("(\"{pair}\", {contract_value}_f64),");
        }
    }

    #[test]
    #[ignore]
    fn linear_option() {
        let mapping = fetch_contract_size(LINEAR_OPTION_URL);
        for (pair, contract_value) in mapping {
            println!("(\"{pair}\", {contract_value}_f64),");
        }
    }
}
