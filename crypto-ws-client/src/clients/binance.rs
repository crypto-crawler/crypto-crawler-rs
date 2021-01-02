use crate::WSClient;
use std::collections::HashMap;

use super::ws_client_internal::{MiscMessage, WSClientInternal};
use log::*;
use serde_json::Value;

pub(super) const EXCHANGE_NAME: &str = "Binance";

const SPOT_WEBSOCKET_URL: &str = "wss://stream.binance.com:9443/stream";
const FUTURES_WEBSOCKET_URL: &str = "wss://fstream.binance.com/stream";
const DELIVERY_WEBSOCKET_URL: &str = "wss://dstream.binance.com/stream";

/// Binance Spot market.
///
///   * WebSocket API doc: <https://binance-docs.github.io/apidocs/spot/en/>
///   * Trading at: <https://www.binance.com/en/trade/BTC_USDT>
pub struct BinanceSpotWSClient<'a> {
    client: WSClientInternal<'a>,
}

/// Binance Coin-margined Future market.
///
///   * WebSocket API doc: <https://binance-docs.github.io/apidocs/delivery/en/>
///   * Trading at: <https://www.binance.com/en/delivery/btcusd_quarter>
pub struct BinanceFutureWSClient<'a> {
    client: WSClientInternal<'a>,
}

/// Binance USDT-margined Perpetual Swap market.
///
///   * WebSocket API doc: <https://binance-docs.github.io/apidocs/futures/en/>
///   * Trading at: <https://www.binance.com/en/futures/BTC_USDT>
pub struct BinanceLinearSwapWSClient<'a> {
    client: WSClientInternal<'a>,
}

/// Binance Coin-margined Perpetual Swap market
///
///   * WebSocket API doc: <https://binance-docs.github.io/apidocs/delivery/en/>
///   * Trading at: <https://www.binance.com/en/delivery/btcusd_perpetual>
pub struct BinanceInverseSwapWSClient<'a> {
    client: WSClientInternal<'a>,
}

fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
    vec![format!(
        r#"{{"id":9527,"method":"{}","params":{}}}"#,
        if subscribe {
            "SUBSCRIBE"
        } else {
            "UNSUBSCRIBE"
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

    if let Some(result) = obj.get("result") {
        if serde_json::Value::Null == *result {
            return MiscMessage::Misc;
        }
    }

    if !obj.contains_key("stream") || !obj.contains_key("data") {
        warn!("Received {} from {}", msg, EXCHANGE_NAME);
        return MiscMessage::Misc;
    }

    MiscMessage::Normal
}

define_client!(
    BinanceSpotWSClient,
    EXCHANGE_NAME,
    SPOT_WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg
);
define_client!(
    BinanceFutureWSClient,
    EXCHANGE_NAME,
    DELIVERY_WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg
);
define_client!(
    BinanceLinearSwapWSClient,
    EXCHANGE_NAME,
    FUTURES_WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg
);
define_client!(
    BinanceInverseSwapWSClient,
    EXCHANGE_NAME,
    DELIVERY_WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg
);

#[cfg(test)]
mod tests {
    #[test]
    fn test_one_channel() {
        let commands = super::channels_to_commands(&vec!["btcusdt@aggTrade".to_string()], true);
        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"id":9527,"method":"SUBSCRIBE","params":["btcusdt@aggTrade"]}"#,
            commands[0]
        );
    }

    #[test]
    fn test_two_channels() {
        let commands = super::channels_to_commands(
            &vec!["btcusdt@aggTrade".to_string(), "btcusdt@ticker".to_string()],
            true,
        );
        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"id":9527,"method":"SUBSCRIBE","params":["btcusdt@aggTrade","btcusdt@ticker"]}"#,
            commands[0]
        );
    }
}
