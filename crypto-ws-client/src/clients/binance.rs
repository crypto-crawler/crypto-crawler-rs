use crate::WSClient;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

use super::utils::ensure_frame_size;
use super::{
    ws_client_internal::{MiscMessage, WSClientInternal},
    Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO,
};
use log::*;
use serde_json::Value;

pub(super) const EXCHANGE_NAME: &str = "binance";

const SPOT_WEBSOCKET_URL: &str = "wss://stream.binance.com:9443/stream";
const LINEAR_WEBSOCKET_URL: &str = "wss://fstream.binance.com/stream";
const INVERSE_WEBSOCKET_URL: &str = "wss://dstream.binance.com/stream";

// https://binance-docs.github.io/apidocs/futures/en/#websocket-market-streams
// A single connection can listen to a maximum of 200 streams
const MAX_NUM_CHANNELS: usize = 200;

// The websocket server will send a ping frame every 5 minutes
const SERVER_PING_INTERVAL: u64 = 300;

// the websocket message size should not exceed 4096 bytes, otherwise
// you'll get `code: 3001, reason: illegal request`
const WS_FRAME_SIZE: usize = 4096;

// Internal unified client
struct BinanceWSClient {
    client: WSClientInternal,
}

/// Binance Spot market.
///
///   * WebSocket API doc: <https://binance-docs.github.io/apidocs/spot/en/>
///   * Trading at: <https://www.binance.com/en/trade/BTC_USDT>
pub struct BinanceSpotWSClient {
    client: BinanceWSClient,
}

/// Binance Coin-margined Future and Swap markets.
///
///   * WebSocket API doc: <https://binance-docs.github.io/apidocs/delivery/en/>
///   * Trading at: <https://www.binance.com/en/delivery/btcusd_quarter>
pub struct BinanceInverseWSClient {
    client: BinanceWSClient,
}

/// Binance USDT-margined Future and Swap markets.
///
///   * WebSocket API doc: <https://binance-docs.github.io/apidocs/futures/en/>
///   * Trading at: <https://www.binance.com/en/futures/BTC_USDT>
pub struct BinanceLinearWSClient {
    client: BinanceWSClient,
}

impl BinanceWSClient {
    fn new(url: &str, tx: Sender<String>) -> Self {
        BinanceWSClient {
            client: WSClientInternal::new(
                EXCHANGE_NAME,
                url,
                tx,
                Self::on_misc_msg,
                Self::channels_to_commands,
                None,
                Some(SERVER_PING_INTERVAL),
            ),
        }
    }

    fn topics_to_command(chunk: &[String], subscribe: bool) -> String {
        format!(
            r#"{{"id":9527,"method":"{}","params":{}}}"#,
            if subscribe {
                "SUBSCRIBE"
            } else {
                "UNSUBSCRIBE"
            },
            serde_json::to_string(chunk).unwrap()
        )
    }

    fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
        ensure_frame_size(
            channels,
            subscribe,
            Self::topics_to_command,
            WS_FRAME_SIZE,
            Some(MAX_NUM_CHANNELS),
        )
    }

    fn on_misc_msg(msg: &str) -> MiscMessage {
        let resp = serde_json::from_str::<HashMap<String, Value>>(msg);
        if resp.is_err() {
            error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
            return MiscMessage::Misc;
        }
        let obj = resp.unwrap();

        if obj.contains_key("error") {
            panic!("Received {} from {}", msg, EXCHANGE_NAME);
        } else if obj.contains_key("stream") && obj.contains_key("data") {
            MiscMessage::Normal
        } else {
            if let Some(result) = obj.get("result") {
                if serde_json::Value::Null != *result {
                    panic!("Received {} from {}", msg, EXCHANGE_NAME);
                } else {
                    info!("Received {} from {}", msg, EXCHANGE_NAME);
                }
            } else {
                warn!("Received {} from {}", msg, EXCHANGE_NAME);
            }
            MiscMessage::Misc
        }
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
impl_trait!(OrderBookTopK, BinanceWSClient, subscribe_orderbook_topk, "depth5", to_raw_channel);

fn to_candlestick_raw_channel(pair: &str, interval: usize) -> String {
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
        impl $struct_name {
            /// Creates a Binance websocket client.
            ///
            /// # Arguments
            ///
            /// * `on_msg` - A callback function to process websocket messages
            /// * `url` - Optional server url, usually you don't need specify it
            pub fn new(tx: Sender<String>, url: Option<&str>) -> Self {
                let real_url = match url {
                    Some(endpoint) => endpoint,
                    None => $default_url,
                };
                $struct_name {
                    client: BinanceWSClient::new(real_url, tx),
                }
            }
        }

        impl WSClient for $struct_name {
            fn subscribe_trade(&self, channels: &[String]) {
                <$struct_name as Trade>::subscribe_trade(self, channels);
            }

            fn subscribe_orderbook(&self, channels: &[String]) {
                <$struct_name as OrderBook>::subscribe_orderbook(self, channels);
            }

            fn subscribe_orderbook_topk(&self, channels: &[String]) {
                <$struct_name as OrderBookTopK>::subscribe_orderbook_topk(self, channels);
            }

            fn subscribe_l3_orderbook(&self, channels: &[String]) {
                <$struct_name as Level3OrderBook>::subscribe_l3_orderbook(self, channels);
            }

            fn subscribe_ticker(&self, channels: &[String]) {
                <$struct_name as Ticker>::subscribe_ticker(self, channels);
            }

            fn subscribe_bbo(&self, channels: &[String]) {
                <$struct_name as BBO>::subscribe_bbo(self, channels);
            }

            fn subscribe_candlestick(&self, symbol_interval_list: &[(String, usize)]) {
                <$struct_name as Candlestick>::subscribe_candlestick(self, symbol_interval_list);
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
        impl Trade for $struct_name {
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
        impl Ticker for $struct_name {
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
        impl BBO for $struct_name {
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
        impl OrderBook for $struct_name {
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
        impl OrderBookTopK for $struct_name {
            fn subscribe_orderbook_topk(&self, pairs: &[String]) {
                self.client.subscribe_orderbook_topk(pairs);
            }
        }
    };
}

impl_orderbook_snapshot!(BinanceSpotWSClient);
impl_orderbook_snapshot!(BinanceInverseWSClient);
impl_orderbook_snapshot!(BinanceLinearWSClient);

macro_rules! impl_candlestick {
    ($struct_name:ident) => {
        impl Candlestick for $struct_name {
            fn subscribe_candlestick(&self, symbol_interval_list: &[(String, usize)]) {
                self.client.subscribe_candlestick(symbol_interval_list);
            }
        }
    };
}

impl_candlestick!(BinanceSpotWSClient);
impl_candlestick!(BinanceInverseWSClient);
impl_candlestick!(BinanceLinearWSClient);

panic_l3_orderbook!(BinanceSpotWSClient);
panic_l3_orderbook!(BinanceInverseWSClient);
panic_l3_orderbook!(BinanceLinearWSClient);

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
