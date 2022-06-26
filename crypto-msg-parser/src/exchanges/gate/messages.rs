use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// https://www.gateio.pro/docs/apiv4/ws/en/#server-response
// https://www.gateio.pro/docs/futures/ws/en/#response
// https://www.gateio.pro/docs/delivery/ws/en/#response
#[derive(Serialize, Deserialize)]
pub(super) struct WebsocketMsg<T: Sized> {
    pub time: i64,
    pub channel: String,
    pub event: String,
    pub error: Option<Value>,
    pub result: T,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

// https://www.gateio.pro/docs/apiv4/ws/en/#server-response
// https://www.gate.io/docs/developers/apiv4/ws/en/#best-bid-or-ask-price
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub(super) struct RawBboMsg {
    pub t: Option<i64>,         // Order book update time in milliseconds
    pub u: u64,         // Order book update ID
    pub s: String,      // Currency pair
    pub b: String,      // best bid price
    pub B: String,      // best bid amount
    pub a: String,      // best ask price
    pub A: String,      // best ask amount
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}