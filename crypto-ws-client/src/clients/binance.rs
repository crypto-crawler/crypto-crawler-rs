use crate::WSClient;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::{
    ws_client_internal::{MiscMessage, WSClientInternal},
    Candlestick, OrderBook, OrderBookSnapshot, Ticker, Trade, BBO,
};
use log::*;
use serde_json::Value;

pub(super) const EXCHANGE_NAME: &str = "binance";

const SPOT_WEBSOCKET_URL: &str = "wss://stream.binance.com:9443/stream";
const LINEAR_WEBSOCKET_URL: &str = "wss://fstream.binance.com/stream";
const INVERSE_WEBSOCKET_URL: &str = "wss://dstream.binance.com/stream";

// A single connection can listen to a maximum of 200 streams
const MAX_NUM_CHANNELS: usize = 200;

// The websocket server will send a ping frame every 5 minutes
const SERVER_PING_INTERVAL: u64 = 300;

// Internal unified client
struct BinanceWSClient<'a> {
    client: WSClientInternal<'a>,
}

/// Binance Spot market.
///
///   * WebSocket API doc: <https://binance-docs.github.io/apidocs/spot/en/>
///   * Trading at: <https://www.binance.com/en/trade/BTC_USDT>
pub struct BinanceSpotWSClient<'a> {
    client: BinanceWSClient<'a>,
}

/// Binance Coin-margined Future and Swap markets.
///
///   * WebSocket API doc: <https://binance-docs.github.io/apidocs/delivery/en/>
///   * Trading at: <https://www.binance.com/en/delivery/btcusd_quarter>
pub struct BinanceInverseWSClient<'a> {
    client: BinanceWSClient<'a>,
}

/// Binance USDT-margined Future and Swap markets.
///
///   * WebSocket API doc: <https://binance-docs.github.io/apidocs/futures/en/>
///   * Trading at: <https://www.binance.com/en/futures/BTC_USDT>
pub struct BinanceLinearWSClient<'a> {
    client: BinanceWSClient<'a>,
}

impl<'a> BinanceWSClient<'a> {
    fn new(url: &str, on_msg: Arc<Mutex<dyn FnMut(String) + 'a + Send>>) -> Self {
        BinanceWSClient {
            client: WSClientInternal::new(
                EXCHANGE_NAME,
                url,
                on_msg,
                Self::on_misc_msg,
                Self::channels_to_commands,
                None,
                Some(SERVER_PING_INTERVAL),
            ),
        }
    }

    fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
        let raw_channels: Vec<&String> =
            channels.iter().filter(|ch| !ch.starts_with('{')).collect();
        let mut all_commands: Vec<String> = channels
            .iter()
            .filter(|ch| ch.starts_with('{'))
            .map(|s| s.to_string())
            .collect();

        if !raw_channels.is_empty() {
            let n = raw_channels.len();
            for i in (0..n).step_by(MAX_NUM_CHANNELS) {
                let chunk: Vec<&String> =
                    (&raw_channels[i..(std::cmp::min(i + MAX_NUM_CHANNELS, n))]).to_vec();
                let command = format!(
                    r#"{{"id":9527,"method":"{}","params":{}}}"#,
                    if subscribe {
                        "SUBSCRIBE"
                    } else {
                        "UNSUBSCRIBE"
                    },
                    serde_json::to_string(&chunk).unwrap()
                );
                all_commands.push(command);
            }
        };

        all_commands
    }

    fn on_misc_msg(msg: &str) -> MiscMessage {
        let resp = serde_json::from_str::<HashMap<String, Value>>(msg);
        if resp.is_err() {
            error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
            return MiscMessage::Misc;
        }
        let obj = resp.unwrap();

        if obj.contains_key("error") {
            error!("Received {} from {}", msg, EXCHANGE_NAME);
            panic!("Received {} from {}", msg, EXCHANGE_NAME);
        }

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
}

fn to_raw_channel(channel: &str, pair: &str) -> String {
    format!("{}@{}", pair.to_lowercase(), channel)
}

#[rustfmt::skip]
impl_trait!(Trade, BinanceWSClient, subscribe_trade, "aggTrade", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, BinanceWSClient, subscribe_ticker, "ticker", to_raw_channel);
#[rustfmt::skip]
impl_trait!(BBO, BinanceWSClient, subscribe_bbo, "bookTicker", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBook, BinanceWSClient, subscribe_orderbook, "depth", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBookSnapshot, BinanceWSClient, subscribe_orderbook_snapshot, "depth20", to_raw_channel);

fn to_candlestick_raw_channel(pair: &str, interval: u32) -> String {
    let interval_str = match interval {
        60 => "1m",
        180 => "3m",
        300 => "5m",
        900 => "15m",
        1800 => "30m",
        3600 => "1h",
        7200 => "2h",
        14400 => "4h",
        21600 => "6h",
        28800 => "8h",
        43200 => "12h",
        86400 => "1d",
        259200 => "3d",
        604800 => "1w",
        2592000 => "1M",
        _ => panic!("Binance has intervals 1m,3m,5m,15m,30m,1h,2h,4h,6h,8h,12h,1d,3d,1w,1M"),
    };
    format!("{}@kline_{}", pair, interval_str)
}

impl_candlestick!(BinanceWSClient);

/// Define market specific client.
macro_rules! define_market_client {
    ($struct_name:ident, $default_url:ident) => {
        impl<'a> WSClient<'a> for $struct_name<'a> {
            fn new(on_msg: Arc<Mutex<dyn FnMut(String) + 'a + Send>>, url: Option<&str>) -> Self {
                let real_url = match url {
                    Some(endpoint) => endpoint,
                    None => $default_url,
                };
                $struct_name {
                    client: BinanceWSClient::new(real_url, on_msg),
                }
            }

            fn subscribe_trade(&self, channels: &[String]) {
                <$struct_name as Trade>::subscribe_trade(self, channels);
            }

            fn subscribe_orderbook(&self, channels: &[String]) {
                <$struct_name as OrderBook>::subscribe_orderbook(self, channels);
            }

            fn subscribe_orderbook_snapshot(&self, channels: &[String]) {
                <$struct_name as OrderBookSnapshot>::subscribe_orderbook_snapshot(self, channels);
            }

            fn subscribe_ticker(&self, channels: &[String]) {
                <$struct_name as Ticker>::subscribe_ticker(self, channels);
            }

            fn subscribe_bbo(&self, channels: &[String]) {
                <$struct_name as BBO>::subscribe_bbo(self, channels);
            }

            fn subscribe_candlestick(&self, pairs: &[String], interval: u32) {
                <$struct_name as Candlestick>::subscribe_candlestick(self, pairs, interval);
            }

            fn subscribe(&self, channels: &[String]) {
                self.client.client.subscribe(channels);
            }

            fn unsubscribe(&self, channels: &[String]) {
                self.client.client.unsubscribe(channels);
            }

            fn run(&self, duration: Option<u64>) {
                self.client.client.run(duration);
            }

            fn close(&self) {
                self.client.client.close();
            }
        }
    };
}

define_market_client!(BinanceSpotWSClient, SPOT_WEBSOCKET_URL);
define_market_client!(BinanceInverseWSClient, INVERSE_WEBSOCKET_URL);
define_market_client!(BinanceLinearWSClient, LINEAR_WEBSOCKET_URL);

macro_rules! impl_trade {
    ($struct_name:ident) => {
        impl<'a> Trade for $struct_name<'a> {
            fn subscribe_trade(&self, pairs: &[String]) {
                self.client.subscribe_trade(pairs);
            }
        }
    };
}

impl_trade!(BinanceSpotWSClient);
impl_trade!(BinanceInverseWSClient);
impl_trade!(BinanceLinearWSClient);

macro_rules! impl_ticker {
    ($struct_name:ident) => {
        impl<'a> Ticker for $struct_name<'a> {
            fn subscribe_ticker(&self, pairs: &[String]) {
                self.client.subscribe_ticker(pairs);
            }
        }
    };
}

impl_ticker!(BinanceSpotWSClient);
impl_ticker!(BinanceInverseWSClient);
impl_ticker!(BinanceLinearWSClient);

macro_rules! impl_bbo {
    ($struct_name:ident) => {
        impl<'a> BBO for $struct_name<'a> {
            fn subscribe_bbo(&self, pairs: &[String]) {
                self.client.subscribe_bbo(pairs);
            }
        }
    };
}

impl_bbo!(BinanceSpotWSClient);
impl_bbo!(BinanceInverseWSClient);
impl_bbo!(BinanceLinearWSClient);

macro_rules! impl_orderbook {
    ($struct_name:ident) => {
        impl<'a> OrderBook for $struct_name<'a> {
            fn subscribe_orderbook(&self, pairs: &[String]) {
                self.client.subscribe_orderbook(pairs);
            }
        }
    };
}

impl_orderbook!(BinanceSpotWSClient);
impl_orderbook!(BinanceInverseWSClient);
impl_orderbook!(BinanceLinearWSClient);

macro_rules! impl_orderbook_snapshot {
    ($struct_name:ident) => {
        impl<'a> OrderBookSnapshot for $struct_name<'a> {
            fn subscribe_orderbook_snapshot(&self, pairs: &[String]) {
                self.client.subscribe_orderbook_snapshot(pairs);
            }
        }
    };
}

impl_orderbook_snapshot!(BinanceSpotWSClient);
impl_orderbook_snapshot!(BinanceInverseWSClient);
impl_orderbook_snapshot!(BinanceLinearWSClient);

macro_rules! impl_candlestick {
    ($struct_name:ident) => {
        impl<'a> Candlestick for $struct_name<'a> {
            fn subscribe_candlestick(&self, pairs: &[String], interval: u32) {
                self.client.subscribe_candlestick(pairs, interval);
            }
        }
    };
}

impl_candlestick!(BinanceSpotWSClient);
impl_candlestick!(BinanceInverseWSClient);
impl_candlestick!(BinanceLinearWSClient);

#[cfg(test)]
mod tests {
    #[test]
    fn test_one_channel() {
        let commands = super::BinanceWSClient::channels_to_commands(
            &vec!["btcusdt@aggTrade".to_string()],
            true,
        );
        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"id":9527,"method":"SUBSCRIBE","params":["btcusdt@aggTrade"]}"#,
            commands[0]
        );
    }

    #[test]
    fn test_two_channels() {
        let commands = super::BinanceWSClient::channels_to_commands(
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
