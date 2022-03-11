use std::collections::HashMap;
use std::sync::mpsc::Sender;

use crate::WSClient;

use super::super::{
    utils::CHANNEL_PAIR_DELIMITER,
    ws_client_internal::{MiscMessage, WSClientInternal},
    Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO,
};
use super::EXCHANGE_NAME;

use log::*;
use serde_json::Value;

pub(super) const SWAP_WEBSOCKET_URL: &str = "wss://contract.mexc.com/ws";

// more than 60 seconds no response, close the channel
const SWAP_CLIENT_PING_INTERVAL_AND_MSG: (u64, &str) = (60, r#"{"method":"ping"}"#);

/// MEXC Swap market.
///
///   * WebSocket API doc: <https://mxcdevelop.github.io/APIDoc/contract.api.en.html#websocket-api>
///   * Trading at: <https://contract.mexc.com/exchange>
pub struct MexcSwapWSClient {
    client: WSClientInternal,
}

// Example: deal:BTC_USDT -> {"method":"sub.deal","param":{"symbol":"BTC_USDT"}}
fn channel_to_command(ch: &str, subscribe: bool) -> String {
    if ch.starts_with('{') {
        return ch.to_string();
    }
    let v: Vec<&str> = ch.split(CHANNEL_PAIR_DELIMITER).collect();
    let channel = v[0];
    let symbol = v[1];
    format!(
        r#"{{"method":"{}.{}","param":{{"symbol":"{}"}}}}"#,
        if subscribe { "sub" } else { "unsub" },
        channel,
        symbol
    )
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
    let obj = serde_json::from_str::<HashMap<String, Value>>(msg).unwrap();
    if obj.contains_key("channel") && obj.contains_key("data") && obj.contains_key("ts") {
        let channel = obj.get("channel").unwrap().as_str().unwrap();
        match channel {
            "pong" => MiscMessage::Pong,
            "rs.error" => {
                error!("Received {} from {}", msg, EXCHANGE_NAME);
                panic!("Received {} from {}", msg, EXCHANGE_NAME);
            }
            _ => {
                if obj.contains_key("symbol") && channel.starts_with("push.") {
                    MiscMessage::Normal
                } else {
                    info!("Received {} from {}", msg, EXCHANGE_NAME);
                    MiscMessage::Misc
                }
            }
        }
    } else {
        error!("Received {} from {}", msg, SWAP_WEBSOCKET_URL);
        MiscMessage::Misc
    }
}

fn to_raw_channel(channel: &str, symbol: &str) -> String {
    format!("{}{}{}", channel, CHANNEL_PAIR_DELIMITER, symbol)
}

#[rustfmt::skip]
impl_trait!(Trade, MexcSwapWSClient, subscribe_trade, "deal", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, MexcSwapWSClient, subscribe_ticker, "ticker", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, MexcSwapWSClient, subscribe_orderbook, "depth", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBookTopK, MexcSwapWSClient, subscribe_orderbook_topk, "depth.full", to_raw_channel);

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

impl Candlestick for MexcSwapWSClient {
    fn subscribe_candlestick(&self, symbol_interval_list: &[(String, usize)]) {
        let channels = symbol_interval_list
            .iter()
            .map(|(symbol, interval)| {
                format!(
                    r#"{{"method":"sub.kline","param":{{"symbol":"{}","interval":"{}"}}}}"#,
                    symbol,
                    interval_to_string(*interval)
                )
            })
            .collect::<Vec<String>>();

        self.client.subscribe(&channels);
    }
}

panic_bbo!(MexcSwapWSClient);
panic_l3_orderbook!(MexcSwapWSClient);

impl_new_constructor!(
    MexcSwapWSClient,
    EXCHANGE_NAME,
    SWAP_WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg,
    Some(SWAP_CLIENT_PING_INTERVAL_AND_MSG),
    None
);
impl_ws_client_trait!(MexcSwapWSClient);

#[cfg(test)]
mod tests {
    #[test]
    fn test_channel_to_command() {
        let channel = "deal:BTC_USDT";

        let subscribe_command = super::channel_to_command(channel, true);
        assert_eq!(
            r#"{"method":"sub.deal","param":{"symbol":"BTC_USDT"}}"#.to_string(),
            subscribe_command
        );

        let unsubscribe_command = super::channel_to_command(channel, false);
        assert_eq!(
            r#"{"method":"unsub.deal","param":{"symbol":"BTC_USDT"}}"#.to_string(),
            unsubscribe_command
        );
    }
}
