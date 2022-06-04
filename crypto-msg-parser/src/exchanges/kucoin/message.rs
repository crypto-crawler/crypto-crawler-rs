use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub(super) struct WebsocketMsg<T: Sized> {
    pub subject: String,
    pub topic: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub data: T,
}

#[derive(Serialize, Deserialize)]
pub(super) struct RestfulMsg {
    pub code: String,
    pub data: HashMap<String, Value>,
}
