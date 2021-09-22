use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(super) struct WebsocketMsg<T: Sized> {
    pub ch: String,
    pub ts: i64,
    pub tick: T,
}
