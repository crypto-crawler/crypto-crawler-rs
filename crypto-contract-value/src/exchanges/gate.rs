use crypto_market_type::MarketType;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

use super::utils::http_get;

static CONTRACT_VALUES: Lazy<HashMap<MarketType, HashMap<String, f64>>> = Lazy::new(|| {
    let inverse_swap: HashMap<String, f64> = {
        let mut m: HashMap<String, f64> = vec![
            ("ADA/USD", 0.01_f64),
            ("BCH/USD", 0.000001_f64),
            ("BNB/USD", 0.0000001_f64),
            ("BSV/USD", 0.000001_f64),
            ("BTC/USD", 1_f64),
            ("BTM/USD", 0.001_f64),
            ("BTT/USD", 0.1_f64),
            ("DASH/USD", 0.000001_f64),
            ("EOS/USD", 0.0001_f64),
            ("ETC/USD", 0.0001_f64),
            ("ETH/USD", 0.000001_f64),
            ("HT/USD", 0.0001_f64),
            ("LTC/USD", 0.00001_f64),
            ("MDA/USD", 0.0001_f64),
            ("NEO/USD", 0.00001_f64),
            ("ONT/USD", 0.001_f64),
            ("TRX/USD", 0.01_f64),
            ("WAVES/USD", 0.0001_f64),
            ("XLM/USD", 0.001_f64),
            ("XMR/USD", 0.00001_f64),
            ("XRP/USD", 0.001_f64),
            ("ZEC/USD", 0.000001_f64),
            ("ZRX/USD", 0.001_f64),
        ]
        .into_iter()
        .map(|x| (x.0.to_string(), x.1 as f64))
        .collect();

        let from_online = fetch_quanto_multipliers(INVERSE_SWAP_URL);
        for (pair, contract_value) in &from_online {
            m.insert(pair.clone(), *contract_value);
        }

        m
    };

    let linear_swap: HashMap<String, f64> = {
        // offline data, in case the network is down
        let mut m: HashMap<String, f64> = vec![
            ("1INCH/USDT", 1_f64),
            ("AAVE/USDT", 0.01_f64),
            ("ACA/USDT", 1_f64),
            ("ACH/USDT", 11_f64),
            ("ADA/USDT", 10_f64),
            ("AGLD/USDT", 1_f64),
            ("AIOZ/USDT", 10_f64),
            ("AKRO/USDT", 100_f64),
            ("ALCX/USDT", 0.01_f64),
            ("ALGO/USDT", 10_f64),
            ("ALICE/USDT", 0.1_f64),
            ("ALPHA/USDT", 1_f64),
            ("ALT/USDT", 0.001_f64),
            ("AMPL/USDT", 1_f64),
            ("ANC/USDT", 1_f64),
            ("ANKR/USDT", 10_f64),
            ("ANT/USDT", 0.1_f64),
            ("APE/USDT", 1_f64),
            ("API3/USDT", 1_f64),
            ("AR/USDT", 0.1_f64),
            ("ARPA/USDT", 10_f64),
            ("ASD/USDT", 10_f64),
            ("AST/USDT", 10_f64),
            ("ASTR/USDT", 10_f64),
            ("ATA/USDT", 10_f64),
            ("ATLAS/USDT", 100_f64),
            ("ATOM/USDT", 1_f64),
            ("AUCTION/USDT", 0.1_f64),
            ("AUDIO/USDT", 1_f64),
            ("AVAX/USDT", 1_f64),
            ("AXS/USDT", 0.1_f64),
            ("BADGER/USDT", 0.1_f64),
            ("BAKE/USDT", 0.1_f64),
            ("BAL/USDT", 0.1_f64),
            ("BAND/USDT", 0.1_f64),
            ("BAT/USDT", 10_f64),
            ("BCD/USDT", 0.1_f64),
            ("BCH/USDT", 0.01_f64),
            ("BCHA/USDT", 0.1_f64),
            ("BEAM/USDT", 10_f64),
            ("BEL/USDT", 1_f64),
            ("BICO/USDT", 0.1_f64),
            ("BIT/USDT", 1_f64),
            ("BLZ/USDT", 10_f64),
            ("BNB/USDT", 0.001_f64),
            ("BNT/USDT", 1_f64),
            ("BOBA/USDT", 1_f64),
            ("BSV/USDT", 0.01_f64),
            ("BSW/USDT", 1_f64),
            ("BTC/USDT", 0.0001_f64),
            ("BTM/USDT", 10_f64),
            ("BTS/USDT", 100_f64),
            ("BZZ/USDT", 0.1_f64),
            ("C98/USDT", 1_f64),
            ("CAKE/USDT", 0.1_f64),
            ("CEL/USDT", 1_f64),
            ("CELO/USDT", 1_f64),
            ("CELR/USDT", 10_f64),
            ("CERE/USDT", 10_f64),
            ("CFX/USDT", 10_f64),
            ("CHR/USDT", 10_f64),
            ("CHZ/USDT", 100_f64),
            ("CKB/USDT", 100_f64),
            ("CLV/USDT", 1_f64),
            ("COMP/USDT", 0.01_f64),
            ("CONV/USDT", 10_f64),
            ("COTI/USDT", 1_f64),
            ("CREAM/USDT", 0.01_f64),
            ("CRO/USDT", 1_f64),
            ("CRU/USDT", 0.01_f64),
            ("CRV/USDT", 0.1_f64),
            ("CSPR/USDT", 10_f64),
            ("CTK/USDT", 1_f64),
            ("CTSI/USDT", 1_f64),
            ("CVC/USDT", 10_f64),
            ("CVX/USDT", 0.1_f64),
            ("DAR/USDT", 1_f64),
            ("DASH/USDT", 0.01_f64),
            ("DEFI/USDT", 0.001_f64),
            ("DEGO/USDT", 0.1_f64),
            ("DENT/USDT", 1000_f64),
            ("DGB/USDT", 10_f64),
            ("DIA/USDT", 1_f64),
            ("DODO/USDT", 10_f64),
            ("DOGE/USDT", 10_f64),
            ("DOT/USDT", 1_f64),
            ("DUSK/USDT", 10_f64),
            ("DYDX/USDT", 0.1_f64),
            ("EDEN/USDT", 10_f64),
            ("EGLD/USDT", 0.1_f64),
            ("ENJ/USDT", 1_f64),
            ("ENS/USDT", 0.1_f64),
            ("EOS/USDT", 1_f64),
            ("ETC/USDT", 0.1_f64),
            ("ETH/USDT", 0.01_f64),
            ("EXCH/USDT", 0.001_f64),
            ("FIDA/USDT", 1_f64),
            ("FIL/USDT", 0.01_f64),
            ("FIL6/USDT", 0.1_f64),
            ("FITFI/USDT", 1_f64),
            ("FLM/USDT", 10_f64),
            ("FLOW/USDT", 0.1_f64),
            ("FLUX/USDT", 1_f64),
            ("FRONT/USDT", 1_f64),
            ("FTM/USDT", 1_f64),
            ("FTT/USDT", 0.01_f64),
            ("GAL/USDT", 0.1_f64),
            ("GALA/USDT", 10_f64),
            ("GARI/USDT", 10_f64),
            ("GITCOIN/USDT", 0.1_f64),
            ("GLMR/USDT", 0.1_f64),
            ("GMT/USDT", 1_f64),
            ("GRIN/USDT", 10_f64),
            ("GRT/USDT", 10_f64),
            ("GST/USDT", 0.1_f64),
            ("GT/USDT", 0.1_f64),
            ("HBAR/USDT", 10_f64),
            ("HIGH/USDT", 1_f64),
            ("HIVE/USDT", 1_f64),
            ("HNT/USDT", 0.1_f64),
            ("HOT/USDT", 1000_f64),
            ("HT/USDT", 1_f64),
            ("ICP/USDT", 0.001_f64),
            ("ICX/USDT", 1_f64),
            ("IMX/USDT", 1_f64),
            ("INJ/USDT", 0.1_f64),
            ("IOST/USDT", 10_f64),
            ("IOTA/USDT", 1_f64),
            ("IOTX/USDT", 10_f64),
            ("IRIS/USDT", 10_f64),
            ("JASMY/USDT", 1_f64),
            ("JST/USDT", 100_f64),
            ("KAVA/USDT", 1_f64),
            ("KDA/USDT", 1_f64),
            ("KEEP/USDT", 1_f64),
            ("KIN/USDT", 10000_f64),
            ("KLAY/USDT", 1_f64),
            ("KNC/USDT", 1_f64),
            ("KSM/USDT", 0.1_f64),
            ("LDO/USDT", 1_f64),
            ("LEO/USDT", 0.1_f64),
            ("LINA/USDT", 10_f64),
            ("LINK/USDT", 1_f64),
            ("LIT/USDT", 1_f64),
            ("LOKA/USDT", 1_f64),
            ("LON/USDT", 1_f64),
            ("LOOKS/USDT", 1_f64),
            ("LPT/USDT", 0.1_f64),
            ("LRC/USDT", 1_f64),
            ("LTC/USDT", 0.1_f64),
            ("LUNA/USDT", 1_f64),
            ("LUNC/USDT", 10000_f64),
            ("MAKITA/USDT", 0.1_f64),
            ("MANA/USDT", 0.1_f64),
            ("MAPS/USDT", 1_f64),
            ("MASK/USDT", 0.1_f64),
            ("MATIC/USDT", 10_f64),
            ("MBABYDOGE/USDT", 100_f64),
            ("MBOX/USDT", 0.1_f64),
            ("MCB/USDT", 1_f64),
            ("MELON/USDT", 1_f64),
            ("MER/USDT", 10_f64),
            ("MINA/USDT", 0.1_f64),
            ("MKISHU/USDT", 100_f64),
            ("MKR/USDT", 0.001_f64),
            ("MNGO/USDT", 10_f64),
            ("MOB/USDT", 1_f64),
            ("MOVR/USDT", 0.01_f64),
            ("MTA/USDT", 10_f64),
            ("MTL/USDT", 0.1_f64),
            ("NEAR/USDT", 1_f64),
            ("NEST/USDT", 10_f64),
            ("NEXO/USDT", 1_f64),
            ("NFT/USDT", 100000_f64),
            ("NKN/USDT", 1_f64),
            ("NU/USDT", 1_f64),
            ("NYM/USDT", 1_f64),
            ("OCEAN/USDT", 1_f64),
            ("OGN/USDT", 1_f64),
            ("OKB/USDT", 0.1_f64),
            ("OLDLUNA/USDT", 1_f64),
            ("OMG/USDT", 1_f64),
            ("ONE/USDT", 10_f64),
            ("ONT/USDT", 1_f64),
            ("OOKI/USDT", 100_f64),
            ("OP/USDT", 1_f64),
            ("OXY/USDT", 1_f64),
            ("PEARL/USDT", 0.001_f64),
            ("PEOPLE/USDT", 10_f64),
            ("PERP/USDT", 0.1_f64),
            ("POLIS/USDT", 1_f64),
            ("POLS/USDT", 1_f64),
            ("POLY/USDT", 1_f64),
            ("POND/USDT", 10_f64),
            ("PRIV/USDT", 0.001_f64),
            ("PROM/USDT", 0.1_f64),
            ("PRQ/USDT", 10_f64),
            ("PUNDIX/USDT", 1_f64),
            ("PYR/USDT", 9.6_f64),
            ("QRDO/USDT", 1_f64),
            ("QTUM/USDT", 1_f64),
            ("QUICK/USDT", 0.01_f64),
            ("RACA/USDT", 100_f64),
            ("RAD/USDT", 0.1_f64),
            ("RAMP/USDT", 10_f64),
            ("RAY/USDT", 0.1_f64),
            ("REEF/USDT", 100_f64),
            ("REN/USDT", 10_f64),
            ("REQ/USDT", 1_f64),
            ("RLC/USDT", 1_f64),
            ("RNDR/USDT", 1_f64),
            ("RON/USDT", 1_f64),
            ("ROOK/USDT", 0.01_f64),
            ("ROSE/USDT", 100_f64),
            ("RSR/USDT", 100_f64),
            ("RUNE/USDT", 0.1_f64),
            ("RVN/USDT", 10_f64),
            ("SAND/USDT", 1_f64),
            ("SC/USDT", 100_f64),
            ("SCRT/USDT", 1_f64),
            ("SERO/USDT", 10_f64),
            ("SFP/USDT", 1_f64),
            ("SHIB/USDT", 10000_f64),
            ("SKL/USDT", 10_f64),
            ("SLP/USDT", 1_f64),
            ("SNX/USDT", 0.1_f64),
            ("SOL/USDT", 1_f64),
            ("SOS/USDT", 100000_f64),
            ("SPELL/USDT", 1000_f64),
            ("SRM/USDT", 1_f64),
            ("STEP/USDT", 10_f64),
            ("STG/USDT", 1_f64),
            ("STMX/USDT", 100_f64),
            ("STORJ/USDT", 1_f64),
            ("STX/USDT", 1_f64),
            ("SUN/USDT", 0.1_f64),
            ("SUPER/USDT", 1_f64),
            ("SUSHI/USDT", 1_f64),
            ("SXP/USDT", 1_f64),
            ("TFUEL/USDT", 10_f64),
            ("THETA/USDT", 1_f64),
            ("TLM/USDT", 1_f64),
            ("TOMO/USDT", 1_f64),
            ("TONCOIN/USDT", 0.1_f64),
            ("TRB/USDT", 0.1_f64),
            ("TRIBE/USDT", 1_f64),
            ("TRU/USDT", 10_f64),
            ("TRX/USDT", 100_f64),
            ("TRYB/USDT", 10_f64),
            ("UNFI/USDT", 0.1_f64),
            ("UNI/USDT", 1_f64),
            ("UST/USDT", 1_f64),
            ("USTC/USDT", 100_f64),
            ("VET/USDT", 100_f64),
            ("VRA/USDT", 10_f64),
            ("WAVES/USDT", 1_f64),
            ("WAXP/USDT", 1_f64),
            ("WIN/USDT", 10000_f64),
            ("WOO/USDT", 1_f64),
            ("WSB/USDT", 0.001_f64),
            ("XAUG/USDT", 0.001_f64),
            ("XCH/USDT", 0.001_f64),
            ("XCN/USDT", 10_f64),
            ("XEC/USDT", 10000_f64),
            ("XEM/USDT", 1_f64),
            ("XLM/USDT", 10_f64),
            ("XMR/USDT", 0.01_f64),
            ("XRP/USDT", 10_f64),
            ("XTZ/USDT", 1_f64),
            ("XVS/USDT", 0.01_f64),
            ("YFI/USDT", 0.0001_f64),
            ("YFII/USDT", 0.001_f64),
            ("YGG/USDT", 1_f64),
            ("ZEC/USDT", 0.01_f64),
            ("ZEN/USDT", 0.1_f64),
            ("ZIL/USDT", 10_f64),
            ("ZKS/USDT", 1_f64),
            ("ZRX/USDT", 1_f64),
        ]
        .into_iter()
        .map(|x| (x.0.to_string(), x.1))
        .collect();

        let from_online = fetch_quanto_multipliers(LINEAR_SWAP_URL);
        for (pair, contract_value) in from_online {
            m.insert(pair, contract_value);
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
    result.insert(MarketType::InverseSwap, inverse_swap);
    result.insert(MarketType::LinearSwap, linear_swap);
    result.insert(MarketType::LinearFuture, linear_future);
    result
});

const INVERSE_SWAP_URL: &str = "https://api.gateio.ws/api/v4/futures/btc/contracts";
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

    if let Ok(txt) = http_get(url) {
        if let Ok(markets) = serde_json::from_str::<Vec<RawMarket>>(&txt) {
            for market in markets.iter() {
                let mut contract_value = market.quanto_multiplier.parse::<f64>().unwrap();
                if contract_value == 0.0 {
                    contract_value = 1.0;
                }
                assert!(contract_value > 0.0);
                mapping.insert(
                    crypto_pair::normalize_pair(&market.name, "gate").unwrap(),
                    contract_value,
                );
            }
        }
    }

    mapping
}

pub(crate) fn get_contract_value(market_type: MarketType, pair: &str) -> Option<f64> {
    match market_type {
        MarketType::InverseSwap | MarketType::InverseFuture => Some(1.0),
        MarketType::LinearSwap | MarketType::LinearFuture => {
            Some(CONTRACT_VALUES[&market_type][pair])
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{fetch_quanto_multipliers, INVERSE_SWAP_URL, LINEAR_FUTURE_URL, LINEAR_SWAP_URL};

    #[test]
    fn inverse_swap() {
        let mapping = fetch_quanto_multipliers(INVERSE_SWAP_URL);
        for (pair, contract_value) in &mapping {
            println!("(\"{pair}\", {contract_value}_f64),");
        }
    }

    #[test]
    fn linear_swap() {
        let mapping = fetch_quanto_multipliers(LINEAR_SWAP_URL);
        for (pair, contract_value) in &mapping {
            println!("(\"{pair}\", {contract_value}_f64),");
        }
    }

    #[test]
    fn linear_future() {
        let mapping = fetch_quanto_multipliers(LINEAR_FUTURE_URL);
        for (pair, contract_value) in &mapping {
            println!("(\"{pair}\", {contract_value}_f64),");
        }
    }
}
