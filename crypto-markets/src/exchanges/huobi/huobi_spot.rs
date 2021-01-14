use super::utils::huobi_http_get;
use crate::error::Result;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct SpotMarket {
    base_currency: String,
    quote_currency: String,
    price_precision: f64,
    amount_precision: f64,
    symbol_partition: String,
    symbol: String,
    state: String,
    value_precision: f64,
    min_order_amt: f64,
    max_order_amt: f64,
    min_order_value: f64,
    limit_order_min_order_amt: f64,
    limit_order_max_order_amt: f64,
    sell_market_min_order_amt: f64,
    sell_market_max_order_amt: f64,
    buy_market_max_order_value: f64,
    api_trading: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    status: String,
    data: Vec<SpotMarket>,
}

// see <https://huobiapi.github.io/docs/spot/v1/en/#get-all-supported-trading-symbol>
fn fetch_spot_markets_raw() -> Result<Vec<SpotMarket>> {
    let txt = huobi_http_get("https://api.huobi.pro/v1/common/symbols")?;
    let resp = serde_json::from_str::<Response>(&txt).unwrap();
    Ok(resp.data)
}

pub(super) fn fetch_spot_symbols() -> Result<Vec<String>> {
    let symbols = fetch_spot_markets_raw()?
        .into_iter()
        .filter(|m| m.state == "online")
        .map(|m| m.symbol)
        .collect::<Vec<String>>();
    Ok(symbols)
}
