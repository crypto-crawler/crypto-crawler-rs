use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(super) struct WebsocketMsg<T: Sized> {
    pub subject: String,
    pub topic: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub data: T,
}
