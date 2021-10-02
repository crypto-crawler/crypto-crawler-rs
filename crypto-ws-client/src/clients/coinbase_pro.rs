use crate::{Level3OrderBook, WSClient};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::{
    utils::CHANNEL_PAIR_DELIMITER,
    ws_client_internal::{MiscMessage, WSClientInternal},
    Candlestick, OrderBook, OrderBookTopK, Ticker, Trade, BBO,
};

use log::*;
use serde_json::Value;

pub(super) const EXCHANGE_NAME: &str = "coinbase_pro";

const WEBSOCKET_URL: &str = "wss://ws-feed.pro.coinbase.com";

/// The WebSocket client for CoinbasePro.
///
/// CoinbasePro has only Spot market.
///
///   * WebSocket API doc: <https://docs.pro.coinbase.com/#websocket-feed>
///   * Trading at: <https://pro.coinbase.com/>
pub struct CoinbaseProWSClient<'a> {
    client: WSClientInternal<'a>,
}

fn channel_pairs_to_command(channel: &str, pairs: &[String]) -> String {
    format!(
        r#"{{"name":"{}","product_ids":{}}}"#,
        channel,
        serde_json::to_string(pairs).unwrap(),
    )
}

fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
    let mut all_commands: Vec<String> = channels
        .iter()
        .filter(|ch| ch.starts_with('{'))
        .map(|s| s.to_string())
        .collect();

    let mut channel_pairs = HashMap::<String, Vec<String>>::new();
    for s in channels.iter().filter(|ch| !ch.starts_with('{')) {
        let v: Vec<&str> = s.split(CHANNEL_PAIR_DELIMITER).collect();
        let channel = v[0];
        let pair = v[1];
        match channel_pairs.get_mut(channel) {
            Some(pairs) => pairs.push(pair.to_string()),
            None => {
                channel_pairs.insert(channel.to_string(), vec![pair.to_string()]);
            }
        }
    }

    if !channel_pairs.is_empty() {
        let mut command = String::new();
        command.push_str(
            format!(
                r#"{{"type":"{}","channels": ["#,
                if subscribe {
                    "subscribe"
                } else {
                    "unsubscribe"
                }
            )
            .as_str(),
        );
        for (channel, pairs) in channel_pairs.iter() {
            command.push_str(channel_pairs_to_command(channel, pairs).as_str());
            command.push(',')
        }
        command.pop();
        command.push_str("]}");

        all_commands.push(command);
    }

    all_commands
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    let resp = serde_json::from_str::<HashMap<String, Value>>(msg);
    if resp.is_err() {
        error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
        return MiscMessage::Misc;
    }
    let obj = resp.unwrap();

    match obj.get("type").unwrap().as_str().unwrap() {
        "error" => {
            error!("Received {} from {}", msg, EXCHANGE_NAME);
            if obj.contains_key("reason")
                && obj
                    .get("reason")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .contains("is not a valid product")
            {
                panic!("Received {} from {}", msg, EXCHANGE_NAME);
            } else {
                MiscMessage::Misc
            }
        }
        "subscriptions" => {
            info!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Misc
        }
        "heartbeat" => {
            debug!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Misc
        }
        _ => MiscMessage::Normal,
    }
}

fn to_raw_channel(channel: &str, pair: &str) -> String {
    format!("{}{}{}", channel, CHANNEL_PAIR_DELIMITER, pair)
}

#[rustfmt::skip]
impl_trait!(Trade, CoinbaseProWSClient, subscribe_trade, "matches", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, CoinbaseProWSClient, subscribe_ticker, "ticker", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, CoinbaseProWSClient, subscribe_orderbook, "level2", to_raw_channel);

impl<'a> BBO for CoinbaseProWSClient<'a> {
    fn subscribe_bbo(&self, _pairs: &[String]) {
        panic!("CoinbasePro WebSocket does NOT have BBO channel");
    }
}

impl<'a> OrderBookTopK for CoinbaseProWSClient<'a> {
    fn subscribe_orderbook_topk(&self, _pairs: &[String]) {
        panic!("CoinbasePro does NOT have orderbook snapshot channel");
    }
}

impl<'a> Candlestick for CoinbaseProWSClient<'a> {
    fn subscribe_candlestick(&self, _pairs: &[String], _interval: u32) {
        panic!("CoinbasePro does NOT have candlestick channel");
    }
}

impl<'a> Level3OrderBook for CoinbaseProWSClient<'a> {
    fn subscribe_l3_orderbook(&self, symbols: &[String]) {
        let raw_channels: Vec<String> = symbols
            .iter()
            .map(|symbol| to_raw_channel("full", symbol))
            .collect();
        self.client.subscribe(&raw_channels);
    }
}

define_client!(
    CoinbaseProWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg,
    None,
    None
);

#[cfg(test)]
mod tests {
    #[test]
    fn test_two_pairs() {
        assert_eq!(
            r#"{"name":"matches","product_ids":["BTC-USD","ETH-USD"]}"#,
            super::channel_pairs_to_command(
                "matches",
                &vec!["BTC-USD".to_string(), "ETH-USD".to_string()],
            )
        );
    }
}
