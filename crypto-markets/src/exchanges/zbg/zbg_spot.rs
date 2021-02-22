use std::collections::HashMap;

use super::super::utils::http_get;
use crate::error::{Error, Result};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct SpotMarket {
    symbol: String,
    symbol_partition: String,
    price_precision: i64,
    min_order_amt: String,
    id: String,
    state: String,
    base_currency: String,
    amount_precision: i64,
    max_order_amt: Option<String>,
    quote_currency: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct ResMsg {
    message: String,
    method: Option<String>,
    code: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Response {
    datas: Vec<SpotMarket>,
    resMsg: ResMsg,
}

// See https://zbgapi.github.io/docs/spot/v1/en/#public-get-all-supported-trading-symbols
fn fetch_spot_markets_raw() -> Result<Vec<SpotMarket>> {
    let txt = http_get("https://www.zbg.com/exchange/api/v1/common/symbols", None)?;
    let resp = serde_json::from_str::<Response>(&txt)?;
    if resp.resMsg.code != "1" {
        Err(Error(txt))
    } else {
        let valid: Vec<SpotMarket> = resp
            .datas
            .into_iter()
            .filter(|x| x.state == "online")
            .collect();
        Ok(valid)
    }
}

pub(super) fn fetch_spot_symbols() -> Result<Vec<String>> {
    let markets = fetch_spot_markets_raw()?;
    let symbols: Vec<String> = markets.into_iter().map(|m| m.symbol).collect();
    Ok(symbols)
}
