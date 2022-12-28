use super::utils::huobi_http_get;
use crate::error::Result;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct OptionMarket {
    symbol: String,
    contract_code: String,
    contract_type: String,
    contract_size: f64,
    price_tick: f64,
    delivery_date: String,
    create_date: String,
    contract_status: i64,
    option_right_type: String,
    exercise_price: f64,
    delivery_asset: String,
    quote_asset: String,
    trade_partition: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    status: String,
    data: Vec<OptionMarket>,
    ts: i64,
}

// see <https://huobiapi.github.io/docs/option/v1/en/#query-option-info>
fn fetch_option_markets_raw() -> Result<Vec<OptionMarket>> {
    let txt = huobi_http_get("https://api.hbdm.com/option-api/v1/option_contract_info")?;
    let resp = serde_json::from_str::<Response>(&txt)?;
    let result: Vec<OptionMarket> =
        resp.data.into_iter().filter(|m| m.contract_status == 1).collect();
    Ok(result)
}

pub(super) fn fetch_option_symbols() -> Result<Vec<String>> {
    let symbols = fetch_option_markets_raw()?
        .into_iter()
        .filter(|m| m.contract_status == 1)
        .map(|m| m.contract_code)
        .collect::<Vec<String>>();
    Ok(symbols)
}
