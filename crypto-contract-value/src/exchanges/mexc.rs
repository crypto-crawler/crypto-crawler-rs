use crypto_market_type::MarketType;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

use super::utils::http_get;

static LINEAR_CONTRACT_VALUES: Lazy<HashMap<String, f64>> = Lazy::new(|| {
    // offline data, in case the network is down
    let mut m: HashMap<String, f64> = vec![
        ("1INCH/USDT", 1_f64),
        ("AAVE/USDT", 0.01_f64),
        ("ACA/USDT", 1_f64),
        ("ACH/USDT", 1_f64),
        ("ADA/USDT", 1_f64),
        ("AGLD/USDT", 0.1_f64),
        ("AKITA/USDT", 1000000_f64),
        ("AKRO/USDT", 1_f64),
        ("ALGO/USDT", 1_f64),
        ("ALICE/USDT", 0.1_f64),
        ("ALPHA/USDT", 1_f64),
        ("ANC/USDT", 1_f64),
        ("ANKR/USDT", 1_f64),
        ("ANT/USDT", 1_f64),
        ("ANY/USDT", 0.1_f64),
        ("APE/USDT", 1_f64),
        ("API3/USDT", 1_f64),
        ("APX/USDT", 10_f64),
        ("AR/USDT", 0.1_f64),
        ("ARPA/USDT", 1_f64),
        ("ASTR/USDT", 10_f64),
        ("ATA/USDT", 1_f64),
        ("ATOM/USDT", 0.1_f64),
        ("AUDIO/USDT", 1_f64),
        ("AVAX/USDT", 0.1_f64),
        ("AXS/USDT", 1_f64),
        ("BABY/USDT", 10_f64),
        ("BABYDOGE/USDT", 1000000000_f64),
        ("BADGER/USDT", 0.1_f64),
        ("BAL/USDT", 0.1_f64),
        ("BAND/USDT", 0.1_f64),
        ("BAT/USDT", 1_f64),
        ("BCH/USDT", 0.01_f64),
        ("BEL/USDT", 1_f64),
        ("BICO/USDT", 1_f64),
        ("BIT/USDT", 1_f64),
        ("BLZ/USDT", 10_f64),
        ("BNB/USDT", 0.01_f64),
        ("BOBA/USDT", 1_f64),
        ("BONE/USDT", 10_f64),
        ("BP/USDT", 0.1_f64),
        ("BSV/USDT", 0.01_f64),
        ("BTC/USDT", 0.0001_f64),
        ("BTM/USDT", 10_f64),
        ("BTS/USDT", 1_f64),
        ("BTT/USDT", 1_f64),
        ("BTTC/USDT", 1000000_f64),
        ("BZZ/USDT", 0.1_f64),
        ("C98/USDT", 1_f64),
        ("CAW/USDT", 10000000_f64),
        ("CEEK/USDT", 10_f64),
        ("CEL/USDT", 10_f64),
        ("CELO/USDT", 1_f64),
        ("CELR/USDT", 1_f64),
        ("CFX/USDT", 1_f64),
        ("CHR/USDT", 10_f64),
        ("CHZ/USDT", 1_f64),
        ("COCOS/USDT", 1_f64),
        ("COMP/USDT", 0.01_f64),
        ("COTI/USDT", 1_f64),
        ("CRO/USDT", 10_f64),
        ("CRV/USDT", 0.1_f64),
        ("CSPR/USDT", 1_f64),
        ("CTK/USDT", 1_f64),
        ("CTSI/USDT", 1_f64),
        ("CVC/USDT", 1_f64),
        ("DAR/USDT", 1_f64),
        ("DASH/USDT", 0.01_f64),
        ("DENT/USDT", 1_f64),
        ("DERI/USDT", 10_f64),
        ("DGB/USDT", 1_f64),
        ("DOGE/USDT", 100_f64),
        ("DORA/USDT", 0.1_f64),
        ("DOT/USDT", 0.1_f64),
        ("DUSK/USDT", 10_f64),
        ("DYDX/USDT", 1_f64),
        ("EDEN/USDT", 1_f64),
        ("EGLD/USDT", 0.1_f64),
        ("ELON/USDT", 1000000_f64),
        ("ENJ/USDT", 1_f64),
        ("ENS/USDT", 0.1_f64),
        ("EOS/USDT", 0.1_f64),
        ("ETC/USDT", 0.1_f64),
        ("ETH/USDT", 0.01_f64),
        ("FILECOIN/USDT", 0.01_f64),
        ("FITFI/USDT", 10_f64),
        ("FLM/USDT", 1_f64),
        ("FLOW/USDT", 0.1_f64),
        ("FLUX/USDT", 1_f64),
        ("FODL/USDT", 1_f64),
        ("FREE/USDT", 100_f64),
        ("FTM/USDT", 1_f64),
        ("FTT/USDT", 0.1_f64),
        ("GAL/USDT", 0.1_f64),
        ("GALA/USDT", 10_f64),
        ("GDO/USDT", 10000000_f64),
        ("GODS/USDT", 1_f64),
        ("GOLDFINCH/USDT", 1_f64),
        ("GRT/USDT", 1_f64),
        ("GST/USDT", 1_f64),
        ("GTC/USDT", 0.1_f64),
        ("HBAR/USDT", 1_f64),
        ("HIVE/USDT", 1_f64),
        ("HNT/USDT", 0.1_f64),
        ("HOT/USDT", 1_f64),
        ("HT/USDT", 0.1_f64),
        ("HTD/USDT", 1_f64),
        ("ICP/USDT", 0.01_f64),
        ("ICX/USDT", 1_f64),
        ("ILV/USDT", 0.001_f64),
        ("IMX/USDT", 1_f64),
        ("IOST/USDT", 100_f64),
        ("IOTA/USDT", 1_f64),
        ("IOTX/USDT", 1_f64),
        ("JASMY/USDT", 10_f64),
        ("JOE/USDT", 1_f64),
        ("JST/USDT", 100_f64),
        ("JUSTICE/USDT", 1000_f64),
        ("KAVA/USDT", 0.1_f64),
        ("KDA/USDT", 1_f64),
        ("KEEP/USDT", 1_f64),
        ("KISHU/USDT", 1000000000_f64),
        ("KLAY/USDT", 1_f64),
        ("KNC/USDT", 0.1_f64),
        ("KSM/USDT", 0.01_f64),
        ("LINA/USDT", 1_f64),
        ("LINK/USDT", 0.1_f64),
        ("LIT/USDT", 0.1_f64),
        ("LOOKS/USDT", 1_f64),
        ("LPT/USDT", 0.1_f64),
        ("LRC/USDT", 1_f64),
        ("LTC/USDT", 0.01_f64),
        ("LUNA/USDT", 10000_f64),
        ("LUNANEW/USDT", 1_f64),
        ("LUNC/USDT", 10000_f64),
        ("LUNCH/USDT", 1000_f64),
        ("MANA/USDT", 1_f64),
        ("MASK/USDT", 0.1_f64),
        ("MATIC/USDT", 1_f64),
        ("MBOX/USDT", 1_f64),
        ("MELI/USDT", 10_f64),
        ("MINA/USDT", 1_f64),
        ("MKR/USDT", 0.01_f64),
        ("MTL/USDT", 1_f64),
        ("NEAR/USDT", 1_f64),
        ("NEO/USDT", 0.1_f64),
        ("NFT/USDT", 1000000_f64),
        ("NKN/USDT", 1_f64),
        ("NU/USDT", 1_f64),
        ("NYM/USDT", 1_f64),
        ("O3/USDT", 1_f64),
        ("OCEAN/USDT", 1_f64),
        ("OGN/USDT", 1_f64),
        ("OKB/USDT", 0.1_f64),
        ("OMG/USDT", 0.1_f64),
        ("ONE/USDT", 1_f64),
        ("ONT/USDT", 1_f64),
        ("OP/USDT", 1_f64),
        ("PEOPLE/USDT", 10_f64),
        ("PIT/USDT", 1000000000_f64),
        ("PLA/USDT", 1_f64),
        ("POKT/USDT", 1_f64),
        ("POLYDOGE/USDT", 100000000_f64),
        ("PSP/USDT", 1_f64),
        ("QTUM/USDT", 0.1_f64),
        ("RACA/USDT", 1000_f64),
        ("RAY/USDT", 0.1_f64),
        ("REEF/USDT", 1_f64),
        ("RNDR/USDT", 1_f64),
        ("ROSE/USDT", 10_f64),
        ("RSR/USDT", 1_f64),
        ("RSS3/USDT", 10_f64),
        ("RUNE/USDT", 1_f64),
        ("RVN/USDT", 10_f64),
        ("SAND/USDT", 1_f64),
        ("SC/USDT", 1_f64),
        ("SCRT/USDT", 1_f64),
        ("SDN/USDT", 0.1_f64),
        ("SFP/USDT", 1_f64),
        ("SHIB/USDT", 1000_f64),
        ("SHIT/USDT", 10000000_f64),
        ("SKL/USDT", 1_f64),
        ("SLP/USDT", 1_f64),
        ("SNX/USDT", 0.1_f64),
        ("SOL/USDT", 0.1_f64),
        ("SOS/USDT", 1000000_f64),
        ("SPA/USDT", 100_f64),
        ("SPELL/USDT", 100_f64),
        ("SRM/USDT", 0.1_f64),
        ("SSS/USDT", 0.1_f64),
        ("STARL/USDT", 100000_f64),
        ("STEPN/USDT", 1_f64),
        ("STMX/USDT", 1_f64),
        ("STORJ/USDT", 1_f64),
        ("SUSHI/USDT", 0.1_f64),
        ("SXP/USDT", 1_f64),
        ("THETA/USDT", 1_f64),
        ("TLM/USDT", 1_f64),
        ("TNS/USDT", 1_f64),
        ("TONCOIN/USDT", 1_f64),
        ("TORN/USDT", 0.01_f64),
        ("TRVL/USDT", 1_f64),
        ("TRX/USDT", 10_f64),
        ("UMA/USDT", 0.1_f64),
        ("UMEE/USDT", 10_f64),
        ("UNFI/USDT", 1_f64),
        ("UNI/USDT", 0.1_f64),
        ("UST/USDT", 10_f64),
        ("USTC/USDT", 100_f64),
        ("VET/USDT", 1_f64),
        ("VRA/USDT", 100_f64),
        ("WAVES/USDT", 0.1_f64),
        ("WAXP/USDT", 10_f64),
        ("WOO/USDT", 1_f64),
        ("WRLD/USDT", 10_f64),
        ("X2Y2/USDT", 1_f64),
        ("XCH/USDT", 0.01_f64),
        ("XEC/USDT", 10000_f64),
        ("XEM/USDT", 1_f64),
        ("XLM/USDT", 10_f64),
        ("XMR/USDT", 0.01_f64),
        ("XRP/USDT", 1_f64),
        ("XTZ/USDT", 0.1_f64),
        ("YFI/USDT", 0.0001_f64),
        ("YFII/USDT", 0.0001_f64),
        ("YGG/USDT", 1_f64),
        ("YOOSHI/USDT", 1000000_f64),
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
});

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

    if let Ok(txt) = http_get("https://contract.mexc.com/api/v1/contract/detail") {
        if let Ok(resp) = serde_json::from_str::<ResponseMsg>(&txt) {
            for linear_market in resp.data.iter().filter(|x| x.settleCoin == x.quoteCoin) {
                mapping.insert(
                    crypto_pair::normalize_pair(&linear_market.symbol, "mexc").unwrap(),
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
        MarketType::LinearSwap => Some(*LINEAR_CONTRACT_VALUES.get(pair).expect(pair)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::fetch_linear_contract_sizes;

    #[test]
    fn linear() {
        let mut mapping = fetch_linear_contract_sizes();
        for (key, value) in super::LINEAR_CONTRACT_VALUES.iter() {
            if !mapping.contains_key(key) {
                mapping.insert(key.to_string(), *value);
            }
        }
        for (pair, contract_value) in &mapping {
            println!("(\"{pair}\", {contract_value}_f64),");
        }
    }
}
