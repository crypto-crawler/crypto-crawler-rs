use std::{collections::HashMap, sync::mpsc::Sender};

use crate::WSClient;

use super::super::{
    utils::CHANNEL_PAIR_DELIMITER,
    ws_client_internal::{MiscMessage, WSClientInternal},
    Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO,
};
use super::EXCHANGE_NAME;

use log::*;
use serde_json::Value;

pub(super) const SPOT_WEBSOCKET_URL: &str = "wss://wbs.mexc.com/raw/ws";

const SPOT_CLIENT_PING_INTERVAL_AND_MSG: (u64, &str) = (5, "ping");

/// MEXC Spot market.
///
///   * WebSocket API doc: <https://github.com/mxcdevelop/APIDoc/blob/master/websocket/spot/websocket-api.md>
///   * Trading at: <https://www.mexc.com/exchange/BTC_USDT>
pub struct MexcSpotWSClient {
    client: WSClientInternal,
}

// Example: deal:BTC_USDT -> {"op":"sub.deal.aggregate","symbol":"BTC_USDT"}
fn channel_to_command(raw_channel: &str, subscribe: bool) -> String {
    if raw_channel.starts_with('{') {
        return raw_channel.to_string();
    }
    let v: Vec<&str> = raw_channel.split(CHANNEL_PAIR_DELIMITER).collect();
    let channel = v[0];
    let symbol = v[1];

    if channel == "limit.depth" {
        format!(
            r#"{{"op":"{}.{}","symbol":"{}","depth": 20}}"#,
            if subscribe { "sub" } else { "unsub" },
            channel,
            symbol
        )
    } else {
        format!(
            r#"{{"op":"{}.{}","symbol":"{}"}}"#,
            if subscribe { "sub" } else { "unsub" },
            channel,
            symbol
        )
    }
}

fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
    let mut commands = Vec::<String>::new();

    for s in channels.iter() {
        let command = channel_to_command(s, subscribe);
        commands.push(command);
    }

    commands
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    if msg == "pong" {
        return MiscMessage::Pong;
    }
    if let Ok(obj) = serde_json::from_str::<HashMap<String, Value>>(msg) {
        if obj.contains_key("channel") && obj.contains_key("data") {
            let channel = obj.get("channel").unwrap().as_str().unwrap();
            match channel {
                "push.deal" | "push.depth" | "push.limit.depth" | "push.kline" => {
                    if obj.contains_key("symbol") {
                        MiscMessage::Normal
                    } else {
                        warn!("Received {} from {}", msg, EXCHANGE_NAME);
                        MiscMessage::Misc
                    }
                }
                "push.overview" => MiscMessage::Normal,
                _ => {
                    warn!("Received {} from {}", msg, EXCHANGE_NAME);
                    MiscMessage::Misc
                }
            }
        } else {
            warn!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Misc
        }
    } else {
        warn!("Received {} from {}", msg, EXCHANGE_NAME);
        MiscMessage::Misc
    }
}

fn to_raw_channel(channel: &str, symbol: &str) -> String {
    format!("{}{}{}", channel, CHANNEL_PAIR_DELIMITER, symbol)
}

#[rustfmt::skip]
impl_trait!(Trade, MexcSpotWSClient, subscribe_trade, "deal", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, MexcSpotWSClient, subscribe_orderbook, "depth", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBookTopK, MexcSpotWSClient, subscribe_orderbook_topk, "limit.depth", to_raw_channel);

fn interval_to_string(interval: usize) -> String {
    let tmp = match interval {
        60 => "Min1",
        300 => "Min5",
        900 => "Min15",
        1800 => "Min30",
        3600 => "Min60",
        14400 => "Hour4",
        28800 => "Hour8",
        86400 => "Day1",
        604800 => "Week1",
        2592000 => "Month1",
        _ => panic!("MEXC has intervals Min1,Min5,Min15,Min30,Min60,Hour4,Hour8,Day1,Week1,Month1"),
    };
    tmp.to_string()
}

impl Candlestick for MexcSpotWSClient {
    fn subscribe_candlestick(&self, symbol_interval_list: &[(String, usize)]) {
        let channels = symbol_interval_list
            .iter()
            .map(|(symbol, interval)| {
                format!(
                    r#"{{"op":"sub.kline","symbol":"{}","interval":"{}"}}"#,
                    symbol,
                    interval_to_string(*interval)
                )
            })
            .collect::<Vec<String>>();

        self.client.subscribe(&channels);
    }
}

panic_bbo!(MexcSpotWSClient);
panic_ticker!(MexcSpotWSClient);
panic_l3_orderbook!(MexcSpotWSClient);

impl_new_constructor!(
    MexcSpotWSClient,
    EXCHANGE_NAME,
    SPOT_WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg,
    Some(SPOT_CLIENT_PING_INTERVAL_AND_MSG),
    None
);
impl_ws_client_trait!(MexcSpotWSClient);

#[cfg(test)]
mod tests {
    #[test]
    fn test_channel_to_command() {
        let channel = "deal:BTC_USDT";

        let subscribe_command = super::channel_to_command(channel, true);
        assert_eq!(
            r#"{"op":"sub.deal","symbol":"BTC_USDT"}"#.to_string(),
            subscribe_command
        );

        let unsubscribe_command = super::channel_to_command(channel, false);
        assert_eq!(
            r#"{"op":"unsub.deal","symbol":"BTC_USDT"}"#.to_string(),
            unsubscribe_command
        );
    }
}
