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

// https://www.gateio.pro/docs/developers/apiv4/zh_CN/#%E5%B8%82%E5%9C%BA-k-%E7%BA%BF%E5%9B%BE
#[derive(Serialize, Deserialize)]
pub(super) struct WebsocketCandlest<T: Sized> {
    pub time: i64,
    pub channel: String,
    pub event: String,
    pub result: T,
}
