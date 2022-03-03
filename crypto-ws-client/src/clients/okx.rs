use crate::clients::utils::CHANNEL_PAIR_DELIMITER;
use crate::WSClient;
use std::collections::{BTreeMap, HashMap};
use std::sync::mpsc::Sender;

use super::utils::ensure_frame_size;
use super::ws_client_internal::{MiscMessage, WSClientInternal};
use super::{Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO};

use log::*;
use serde_json::Value;

pub(super) const EXCHANGE_NAME: &str = "okx";

const WEBSOCKET_URL: &str = "wss://ws.okx.com:8443/ws/v5/public";

// https://www.okx.com/docs-v5/en/#websocket-api-connect
const CLIENT_PING_INTERVAL_AND_MSG: (u64, &str) = (30, "ping");

/// https://www.okx.com/docs-v5/en/#websocket-api-subscribe
/// The total length of multiple channels cannot exceed 4096 bytes
const WS_FRAME_SIZE: usize = 4096;

/// The WebSocket client for OKX.
///
/// OKX has Spot, Future, Swap and Option markets.
///
/// * API doc: <https://www.okx.com/docs-v5/en/#websocket-api>
/// * Trading at:
///     * Spot <https://www.okx.com/trade-spot>
///     * Future <https://www.okx.com/trade-futures>
///     * Swap <https://www.okx.com/trade-swap>
///     * Option <https://www.okx.com/trade-option>
pub struct OkxWSClient {
    client: WSClientInternal,
}

fn topics_to_command(chunk: &[String], subscribe: bool) -> String {
    let arr = chunk
        .iter()
        .map(|s| {
            let mut map = BTreeMap::new();
            let v: Vec<&str> = s.split(CHANNEL_PAIR_DELIMITER).collect();
            let channel = v[0];
            let symbol = v[1];
            map.insert("channel".to_string(), channel.to_string());
            map.insert("instId".to_string(), symbol.to_string());
            map
        })
        .collect::<Vec<BTreeMap<String, String>>>();
    format!(
        r#"{{"op":"{}","args":{}}}"#,
        if subscribe {
            "subscribe"
        } else {
            "unsubscribe"
        },
        serde_json::to_string(&arr).unwrap(),
    )
}

fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
    ensure_frame_size(channels, subscribe, topics_to_command, WS_FRAME_SIZE, None)
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    if msg == "pong" {
        return MiscMessage::Pong;
    }
    let resp = serde_json::from_str::<HashMap<String, Value>>(msg);
    if resp.is_err() {
        error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
        return MiscMessage::Misc;
    }
    let obj = resp.unwrap();

    if let Some(event) = obj.get("event") {
        match event.as_str().unwrap() {
            "error" => {
                let error_code = obj
                    .get("code")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .parse::<i64>()
                    .unwrap();
                match error_code {
                    30040 => {
                        // channel doesn't exist, ignore because some symbols don't exist in websocket while they exist in `/v3/instruments`
                        error!("Received {} from {}", msg, EXCHANGE_NAME);
                    }
                    _ => warn!("Received {} from {}", msg, EXCHANGE_NAME),
                }
            }
            "subscribe" => info!("Received {} from {}", msg, EXCHANGE_NAME),
            "unsubscribe" => info!("Received {} from {}", msg, EXCHANGE_NAME),
            _ => warn!("Received {} from {}", msg, EXCHANGE_NAME),
        }
        MiscMessage::Misc
    } else if !obj.contains_key("arg") || !obj.contains_key("data") {
        error!("Received {} from {}", msg, EXCHANGE_NAME);
        MiscMessage::Misc
    } else {
        MiscMessage::Normal
    }
}

fn to_raw_channel(channel: &str, symbol: &str) -> String {
    format!("{}{}{}", channel, CHANNEL_PAIR_DELIMITER, symbol)
}

#[rustfmt::skip]
impl_trait!(Trade, OkxWSClient, subscribe_trade, "trades", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, OkxWSClient, subscribe_ticker, "tickers", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, OkxWSClient, subscribe_orderbook, "books-l2-tbt", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBookTopK, OkxWSClient, subscribe_orderbook_topk, "books5", to_raw_channel);

fn to_candlestick_raw_channel(symbol: &str, interval: usize) -> String {
    let channel = match interval {
        60 => "candle1m",
        180 => "candle3m",
        300 => "candle5m",
        900 => "candle15m",
        1800 => "candle30m",
        3600 => "candle1H",
        7200 => "candle2H",
        14400 => "candle4H",
        21600 => "candle6H",
        43200 => "candle12H",
        86400 => "candle1D",
        172800 => "candle2D",
        259200 => "candle3D",
        432000 => "candle5D",
        604800 => "candle1W",
        2592000 => "candle1M",
        _ => panic!("Huobi has intervals 1min,5min,15min,30min,60min,4hour,1day,1week,1mon"),
    };
    to_raw_channel(channel, symbol)
}

impl_candlestick!(OkxWSClient);

panic_bbo!(OkxWSClient);
panic_l3_orderbook!(OkxWSClient);

impl_new_constructor!(
    OkxWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg,
    Some(CLIENT_PING_INTERVAL_AND_MSG),
    None
);
impl_ws_client_trait!(OkxWSClient);

#[cfg(test)]
mod tests {
    #[test]
    fn test_one_channel() {
        let commands = super::channels_to_commands(&vec!["trades:BTC-USDT".to_string()], true);
        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"op":"subscribe","args":[{"channel":"trades","instId":"BTC-USDT"}]}"#,
            commands[0]
        );
    }

    #[test]
    fn test_two_channel() {
        let commands = super::channels_to_commands(
            &vec![
                "trades:BTC-USDT".to_string(),
                "tickers:BTC-USDT".to_string(),
            ],
            true,
        );
        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"op":"subscribe","args":[{"channel":"trades","instId":"BTC-USDT"},{"channel":"tickers","instId":"BTC-USDT"}]}"#,
            commands[0]
        );
    }
}
