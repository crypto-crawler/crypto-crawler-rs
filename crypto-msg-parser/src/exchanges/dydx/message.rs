use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub(super) struct WebsocketMsg<T: Sized> {
    #[serde(rename = "type")]
    pub type_: String,
    pub connection_id: String,
    pub message_id: i64,
    pub id: String,
    pub channel: String,
    pub contents: T,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}
