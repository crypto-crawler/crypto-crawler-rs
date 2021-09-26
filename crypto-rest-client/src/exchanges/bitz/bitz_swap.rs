use super::super::utils::http_get;
use crate::error::{Error, Result};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

const BASE_URL: &str = "https://apiv2.bitz.com";

/// The RESTful client for BitZ swap markets.
///
/// * RESTful API doc: <https://apidocv2.bitz.plus/en/>
/// * Trading at: <https://swap.bitz.plus/en/>
/// * Rate Limits: <https://apidocv2.bitz.plus/en/#limit>
///   * no more than 30 times within 1 sec
pub struct BitzSwapRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl BitzSwapRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        BitzSwapRestClient {
            _api_key: api_key,
            _api_secret: api_secret,
        }
    }

    /// Get the latest Level2 snapshot of orderbook.
    ///
    /// Top 100 bids and asks are returned.
    ///
    /// For example: <https://apiv2.bitz.com/V2/Market/getContractOrderBook?contractId=101&depth=100>,
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        let symbol_id_map = get_symbol_id_map()?;
        if !symbol_id_map.contains_key(symbol) {
            return Err(Error(format!(
                "Can NOT find contractId for the pair {}",
                symbol
            )));
        }
        let contract_id = symbol_id_map.get(symbol).unwrap();
        gen_api!(format!(
            "/V2/Market/getContractOrderBook?contractId={}&depth=100",
            contract_id
        ))
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct SwapMarket {
    contractId: String, // contract id
    pair: String,       //contract market
    status: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    status: i64,
    msg: String,
    data: Vec<SwapMarket>,
    time: i64,
    microtime: String,
    source: String,
}

fn get_symbol_id_map() -> Result<HashMap<String, String>> {
    let params = HashMap::new();
    let txt = http_get("https://apiv2.bitz.com/Market/getContractCoin", &params)?;
    let resp = serde_json::from_str::<Response>(&txt)?;
    if resp.status != 200 {
        return Err(Error(txt));
    }

    let mut symbol_id_map = HashMap::<String, String>::new();
    for x in resp.data.iter() {
        if x.status == "1" {
            symbol_id_map.insert(x.pair.clone(), x.contractId.clone());
        }
    }
    Ok(symbol_id_map)
}
