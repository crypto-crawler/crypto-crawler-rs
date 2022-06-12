use crypto_market_type::MarketType;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};

use super::utils::http_get;

static LINEAR_CONTRACT_VALUES: Lazy<HashMap<String, f64>> = Lazy::new(|| {
    // offline data, in case the network is down
    let mut m: HashMap<String, f64> = vec![
        ("1INCH/USDT", 1_f64),
        ("AAVE/USDT", 0.01_f64),
        ("ADA/USDT", 10_f64),
        ("AGLD/USDT", 1_f64),
        ("ALGO/USDT", 1_f64),
        ("ALICE/USDT", 0.1_f64),
        ("APE/USDT", 0.1_f64),
        ("API3/USDT", 0.1_f64),
        ("ARPA/USDT", 10_f64),
        ("ATOM/USDT", 0.1_f64),
        ("AUDIO/USDT", 1_f64),
        ("AVAX/USDT", 0.1_f64),
        ("AXS/USDT", 0.1_f64),
        ("BAL/USDT", 0.1_f64),
        ("BAND/USDT", 0.1_f64),
        ("BAT/USDT", 1_f64),
        ("BCH/USDT", 0.01_f64),
        ("BNB/USDT", 0.01_f64),
        ("BSV/USDT", 0.01_f64),
        ("BTC/USDT", 0.001_f64),
        ("BTT/USDT", 1000_f64),
        ("C98/USDT", 1_f64),
        ("CELO/USDT", 1_f64),
        ("CELR/USDT", 10_f64),
        ("CHR/USDT", 1_f64),
        ("CHZ/USDT", 1_f64),
        ("COMP/USDT", 0.01_f64),
        ("CRO/USDT", 1_f64),
        ("CRV/USDT", 1_f64),
        ("CTSI/USDT", 1_f64),
        ("DASH/USDT", 0.01_f64),
        ("DENT/USDT", 100_f64),
        ("DGB/USDT", 10_f64),
        ("DOGE/USDT", 100_f64),
        ("DOT/USDT", 1_f64),
        ("DYDX/USDT", 0.1_f64),
        ("EGLD/USDT", 0.01_f64),
        ("ENJ/USDT", 1_f64),
        ("ENS/USDT", 0.1_f64),
        ("EOS/USDT", 1_f64),
        ("ETC/USDT", 0.1_f64),
        ("ETH/USDT", 0.01_f64),
        ("FIL/USDT", 0.1_f64),
        ("FLOW/USDT", 0.1_f64),
        ("FTM/USDT", 1_f64),
        ("GAL/USDT", 0.1_f64),
        ("GALA/USDT", 1_f64),
        ("GMT/USDT", 1_f64),
        ("GRT/USDT", 1_f64),
        ("HBAR/USDT", 10_f64),
        ("ICP/USDT", 0.01_f64),
        ("IMX/USDT", 1_f64),
        ("IOST/USDT", 100_f64),
        ("IOTX/USDT", 10_f64),
        ("JASMY/USDT", 10_f64),
        ("KAVA/USDT", 0.1_f64),
        ("KDA/USDT", 0.1_f64),
        ("KEEP/USDT", 1_f64),
        ("KNC/USDT", 1_f64),
        ("KSM/USDT", 0.01_f64),
        ("LINA/USDT", 10_f64),
        ("LINK/USDT", 0.1_f64),
        ("LOOKS/USDT", 1_f64),
        ("LRC/USDT", 1_f64),
        ("LTC/USDT", 0.1_f64),
        ("LUNA/USDT", 1_f64),
        ("LUNC/USDT", 1000_f64),
        ("MANA/USDT", 1_f64),
        ("MATIC/USDT", 10_f64),
        ("MIR/USDT", 0.1_f64),
        ("MKR/USDT", 0.001_f64),
        ("NEAR/USDT", 0.1_f64),
        ("NEO/USDT", 0.1_f64),
        ("NFT/USDT", 100000_f64),
        ("NYM/USDT", 1_f64),
        ("OCEAN/USDT", 1_f64),
        ("OMG/USDT", 0.1_f64),
        ("ONE/USDT", 10_f64),
        ("ONT/USDT", 1_f64),
        ("OP/USDT", 1_f64),
        ("PEOPLE/USDT", 10_f64),
        ("QTUM/USDT", 0.1_f64),
        ("RNDR/USDT", 1_f64),
        ("ROSE/USDT", 10_f64),
        ("RUNE/USDT", 0.1_f64),
        ("RVN/USDT", 10_f64),
        ("SAND/USDT", 1_f64),
        ("SHIB/USDT", 100000_f64),
        ("SLP/USDT", 10_f64),
        ("SNX/USDT", 0.1_f64),
        ("SOL/USDT", 0.1_f64),
        ("SOS/USDT", 100000_f64),
        ("SRM/USDT", 1_f64),
        ("SUSHI/USDT", 1_f64),
        ("SXP/USDT", 1_f64),
        ("THETA/USDT", 0.1_f64),
        ("TLM/USDT", 1_f64),
        ("TRX/USDT", 100_f64),
        ("UNFI/USDT", 0.1_f64),
        ("UNI/USDT", 1_f64),
        ("VET/USDT", 100_f64),
        ("VRA/USDT", 10_f64),
        ("WAVES/USDT", 0.1_f64),
        ("WAXP/USDT", 1_f64),
        ("XCN/USDT", 10_f64),
        ("XEM/USDT", 1_f64),
        ("XLM/USDT", 10_f64),
        ("XMR/USDT", 0.01_f64),
        ("XRP/USDT", 10_f64),
        ("XTZ/USDT", 1_f64),
        ("YFI/USDT", 0.0001_f64),
        ("YGG/USDT", 0.1_f64),
        ("ZEC/USDT", 0.01_f64),
        ("ZIL/USDT", 10.0),
    ]
    .into_iter()
    .map(|x| (x.0.to_string(), x.1))
    .collect();

    let from_online = fetch_linear_multipliers();
    for (pair, contract_value) in from_online {
        m.insert(pair, contract_value);
    }

    m
});

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SwapMarket {
    symbol: String,
    multiplier: f64,
    isInverse: bool,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ResponseMsg {
    code: String,
    data: Vec<SwapMarket>,
}

// get the multiplier field from linear markets
fn fetch_linear_multipliers() -> BTreeMap<String, f64> {
    let mut mapping: BTreeMap<String, f64> = BTreeMap::new();

    if let Ok(txt) = http_get("https://api-futures.kucoin.com/api/v1/contracts/active") {
        if let Ok(resp) = serde_json::from_str::<ResponseMsg>(&txt) {
            for swap_market in resp.data.iter().filter(|x| !x.isInverse) {
                mapping.insert(
                    crypto_pair::normalize_pair(&swap_market.symbol, "kucoin").unwrap(),
                    swap_market.multiplier,
                );
            }
        }
    }

    mapping
}

pub(crate) fn get_contract_value(market_type: MarketType, pair: &str) -> Option<f64> {
    match market_type {
        MarketType::InverseSwap | MarketType::InverseFuture => Some(1.0),
        MarketType::LinearSwap => Some(LINEAR_CONTRACT_VALUES[pair]),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::fetch_linear_multipliers;

    #[test]
    fn linear() {
        let mapping = fetch_linear_multipliers();
        for (pair, contract_value) in &mapping {
            println!("(\"{pair}\", {contract_value}_f64),");
        }
    }
}
