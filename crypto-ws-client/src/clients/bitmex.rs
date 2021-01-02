use crate::{Trade, WSClient};
use std::collections::HashMap;

use super::{
    utils::CHANNEL_PAIR_DELIMITER,
    ws_client_internal::{MiscMessage, WSClientInternal},
};
use log::*;
use serde_json::Value;

pub(super) const EXCHANGE_NAME: &str = "BitMEX";

const WEBSOCKET_URL: &str = "wss://www.bitmex.com/realtime";

/// The WebSocket client for BitMEX.
///
/// BitMEX has Swap and Future markets.
///
///   * WebSocket API doc: <https://www.bitmex.com/app/wsAPI>
///   * Trading at: <https://www.bitmex.com/app/trade/>
pub struct BitMEXWSClient<'a> {
    client: WSClientInternal<'a>,
}

fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
    vec![format!(
        r#"{{"op":"{}","args":{}}}"#,
        if subscribe {
            "subscribe"
        } else {
            "unsubscribe"
        },
        serde_json::to_string(channels).unwrap()
    )]
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    let resp = serde_json::from_str::<HashMap<String, Value>>(&msg);
    if resp.is_err() {
        error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
        return MiscMessage::Misc;
    }
    let obj = resp.unwrap();

    if !obj.contains_key("table") {
        warn!("Received {} from {}", msg, EXCHANGE_NAME);
        MiscMessage::Misc
    } else {
        MiscMessage::Normal
    }
}

impl<'a> Trade for BitMEXWSClient<'a> {
    fn subscribe_trade(&mut self, pairs: &[String]) {
        let pair_to_raw_channel =
            |pair: &String| format!("trade{}{}", CHANNEL_PAIR_DELIMITER, pair);

        let channels = pairs
            .iter()
            .map(pair_to_raw_channel)
            .collect::<Vec<String>>();
        self.client.subscribe(&channels);
    }
}

define_client!(
    BitMEXWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg
);

#[cfg(test)]
mod tests {
    #[test]
    fn test_one_channel() {
        let commands = super::channels_to_commands(&vec!["trade:XBTUSD".to_string()], true);
        assert_eq!(1, commands.len());
        assert_eq!(r#"{"op":"subscribe","args":["trade:XBTUSD"]}"#, commands[0]);
    }

    #[test]
    fn test_multiple_channels() {
        let commands = super::channels_to_commands(
            &vec![
                "trade:XBTUSD".to_string(),
                "quote:XBTUSD".to_string(),
                "orderBookL2_25:XBTUSD".to_string(),
                "tradeBin1m:XBTUSD".to_string(),
            ],
            true,
        );
        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"op":"subscribe","args":["trade:XBTUSD","quote:XBTUSD","orderBookL2_25:XBTUSD","tradeBin1m:XBTUSD"]}"#,
            commands[0]
        );
    }
}
