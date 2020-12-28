use std::{thread, time};

use log::*;
use tungstenite::{client::AutoStream, WebSocket};

pub(super) fn connect_with_retry(url: &str) -> WebSocket<AutoStream> {
    let mut res = tungstenite::connect(url);
    let mut count: i8 = 1;
    while res.is_err() && count < 3 {
        debug!("Failed to connect to {}, re-connecting now...", url);
        thread::sleep(time::Duration::from_secs(3));
        res = tungstenite::connect(url);
        count += 1;
    }

    match res {
        Ok((ws_stream, _)) => ws_stream,
        Err(err) => {
            error!("Failed to connect to {}, aborted", url);
            panic!(err)
        }
    }
}
