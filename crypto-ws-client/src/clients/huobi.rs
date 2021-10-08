use crate::WSClient;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

use log::*;
use serde_json::Value;
use tungstenite::Message;

use super::ws_client_internal::{MiscMessage, WSClientInternal};
use super::{Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO};

pub(super) const EXCHANGE_NAME: &str = "huobi";

const SPOT_WEBSOCKET_URL: &str = "wss://api.huobi.pro/ws";
// const FUTURES_WEBSOCKET_URL: &str = "wss://www.hbdm.com/ws";
// const COIN_SWAP_WEBSOCKET_URL: &str = "wss://api.hbdm.com/swap-ws";
// const USDT_SWAP_WEBSOCKET_URL: &str = "wss://api.hbdm.com/linear-swap-ws";
// const OPTION_WEBSOCKET_URL: &str = "wss://api.hbdm.com/option-ws";
const FUTURES_WEBSOCKET_URL: &str = "wss://futures.huobi.com/ws";
const COIN_SWAP_WEBSOCKET_URL: &str = "wss://futures.huobi.com/swap-ws";
const USDT_SWAP_WEBSOCKET_URL: &str = "wss://futures.huobi.com/linear-swap-ws";
const OPTION_WEBSOCKET_URL: &str = "wss://futures.huobi.com/option-ws";

// The server will send a heartbeat every 5 seconds
const SERVER_PING_INTERVAL: u64 = 5;

// Internal unified client
struct HuobiWSClient {
    client: WSClientInternal,
}

/// Huobi Spot market.
///
/// * WebSocket API doc: <https://huobiapi.github.io/docs/spot/v1/en/>
/// * Trading at: <https://www.huobi.com/en-us/exchange/>
pub struct HuobiSpotWSClient {
    client: HuobiWSClient,
}

/// Huobi Future market.
///
/// * WebSocket API doc: <https://huobiapi.github.io/docs/dm/v1/en/>
/// * Trading at: <https://futures.huobi.com/en-us/contract/exchange/>
pub struct HuobiFutureWSClient {
    client: HuobiWSClient,
}

/// Huobi Inverse Swap market.
///
/// Inverse Swap market uses coins like BTC as collateral.
///
/// * WebSocket API doc: <https://huobiapi.github.io/docs/coin_margined_swap/v1/en/>
/// * Trading at: <https://futures.huobi.com/en-us/swap/exchange/>
pub struct HuobiInverseSwapWSClient {
    client: HuobiWSClient,
}

/// Huobi Linear Swap market.
///
/// Linear Swap market uses USDT as collateral.
///
/// * WebSocket API doc: <https://huobiapi.github.io/docs/usdt_swap/v1/en/>
/// * Trading at: <https://futures.huobi.com/en-us/linear_swap/exchange/>
pub struct HuobiLinearSwapWSClient {
    client: HuobiWSClient,
}
/// Huobi Option market.
///
///
/// * WebSocket API doc: <https://huobiapi.github.io/docs/option/v1/en/>
/// * Trading at: <https://futures.huobi.com/en-us/option/exchange/>
pub struct HuobiOptionWSClient {
    client: HuobiWSClient,
}

impl HuobiWSClient {
    fn new(url: &str, tx: Sender<String>) -> Self {
        HuobiWSClient {
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

    fn subscribe(&self, channels: &[String]) {
        self.client.subscribe(channels);
    }

    fn channel_to_command(channel: &str, subscribe: bool) -> String {
        if channel.starts_with('{') {
            channel.to_string()
        } else {
            format!(
                r#"{{"{}":"{}","id":"crypto-ws-client"}}"#,
                if subscribe { "sub" } else { "unsub" },
                channel
            )
        }
    }

    fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
        channels
            .iter()
            .map(|ch| Self::channel_to_command(ch, subscribe))
            .collect()
    }

    fn on_misc_msg(msg: &str) -> MiscMessage {
        let resp = serde_json::from_str::<HashMap<String, Value>>(msg);
        if resp.is_err() {
            error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
            return MiscMessage::Misc;
        }
        let obj = resp.unwrap();

        // Market Heartbeat
        if obj.contains_key("ping") {
            // The server will send a heartbeat every 5 seconds,
            // - Spot <https://huobiapi.github.io/docs/spot/v1/en/#introduction-11>
            // - Future <https://huobiapi.github.io/docs/dm/v1/en/#websocket-heartbeat-and-authentication-interface>
            // - InverseSwap <https://huobiapi.github.io/docs/coin_margined_swap/v1/en/#market-heartbeat>
            // - LinearSwap <https://huobiapi.github.io/docs/usdt_swap/v1/en/#websocket-heartbeat-and-authentication-interface>
            // - Option <https://huobiapi.github.io/docs/option/v1/en/#websocket-heartbeat-and-authentication-interface>
            debug!("Received {} from {}", msg, EXCHANGE_NAME);
            let timestamp = obj.get("ping").unwrap();
            let mut pong_msg = HashMap::<String, &Value>::new();
            pong_msg.insert("pong".to_string(), timestamp);
            let ws_msg = Message::Text(serde_json::to_string(&pong_msg).unwrap());
            return MiscMessage::WebSocket(ws_msg);
        }
        // Order Push Heartbeat
        // https://huobiapi.github.io/docs/usdt_swap/v1/en/#market-heartbeat
        if obj.contains_key("op") && obj.get("op").unwrap().as_str().unwrap() == "ping" {
            debug!("Received {} from {}", msg, EXCHANGE_NAME);
            let mut pong_msg = obj;
            pong_msg.insert("op".to_string(), serde_json::from_str("\"pong\"").unwrap()); // change ping to pong
            let ws_msg = Message::Text(serde_json::to_string(&pong_msg).unwrap());
            return MiscMessage::WebSocket(ws_msg);
        }

        if (obj.contains_key("ch") || obj.contains_key("topic"))
            && obj.contains_key("ts")
            && (obj.contains_key("tick") || obj.contains_key("data"))
        {
            MiscMessage::Normal
        } else {
            if let Some(status) = obj.get("status") {
                match status.as_str().unwrap() {
                    "ok" => info!("Received {} from {}", msg, EXCHANGE_NAME),
                    "error" => {
                        error!("Received {} from {}", msg, EXCHANGE_NAME);
                        let err_msg = obj.get("err-msg").unwrap().as_str().unwrap();
                        if err_msg.starts_with("invalid") {
                            panic!("Received {} from {}", msg, EXCHANGE_NAME);
                        }
                    }
                    _ => warn!("Received {} from {}", msg, EXCHANGE_NAME),
                }
            } else if let Some(op) = obj.get("op") {
                match op.as_str().unwrap() {
                    "sub" | "unsub" => MiscMessage::Misc,
                    "notify" => MiscMessage::Normal,
                    _ => {
                        warn!("Received {} from {}", msg, EXCHANGE_NAME);
                        MiscMessage::Misc
                    }
                };
            } else {
                warn!("Received {} from {}", msg, EXCHANGE_NAME);
            }
            MiscMessage::Misc
        }
    }
}

fn to_raw_channel(channel: &str, pair: &str) -> String {
    format!("market.{}.{}", pair, channel)
}

#[rustfmt::skip]
impl_trait!(Trade, HuobiWSClient, subscribe_trade, "trade.detail", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, HuobiWSClient, subscribe_ticker, "detail", to_raw_channel);
#[rustfmt::skip]
impl_trait!(BBO, HuobiWSClient, subscribe_bbo, "bbo", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBookTopK, HuobiWSClient, subscribe_orderbook_topk, "depth.step7", to_raw_channel);

impl OrderBook for HuobiWSClient {
    fn subscribe_orderbook(&self, pairs: &[String]) {
        let pair_to_raw_channel = |pair: &String| {
            format!(
                r#"{{"sub": "market.{}.depth.size_20.high_freq","data_type":"incremental","id": "crypto-ws-client"}}"#,
                pair
            )
        };

        let channels = pairs
            .iter()
            .map(pair_to_raw_channel)
            .collect::<Vec<String>>();
        self.client.subscribe(&channels);
    }
}

fn to_candlestick_raw_channel(pair: &str, interval: usize) -> String {
    let interval_str = match interval {
        60 => "1min",
        300 => "5min",
        900 => "15min",
        1800 => "30min",
        3600 => "60min",
        14400 => "4hour",
        86400 => "1day",
        604800 => "1week",
        2592000 => "1mon",
        _ => panic!("Huobi has intervals 1min,5min,15min,30min,60min,4hour,1day,1week,1mon"),
    };
    format!("market.{}.kline.{}", pair, interval_str)
}

impl_candlestick!(HuobiWSClient);

/// Define market specific client.
macro_rules! define_market_client {
    ($struct_name:ident, $default_url:ident) => {
        impl $struct_name {
            /// Creates a Huobi websocket client.
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
                    client: HuobiWSClient::new(real_url, tx),
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
                self.client.subscribe(channels);
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

define_market_client!(HuobiSpotWSClient, SPOT_WEBSOCKET_URL);
define_market_client!(HuobiFutureWSClient, FUTURES_WEBSOCKET_URL);
define_market_client!(HuobiInverseSwapWSClient, COIN_SWAP_WEBSOCKET_URL);
define_market_client!(HuobiLinearSwapWSClient, USDT_SWAP_WEBSOCKET_URL);
define_market_client!(HuobiOptionWSClient, OPTION_WEBSOCKET_URL);

macro_rules! impl_trade {
    ($struct_name:ident) => {
        impl Trade for $struct_name {
            fn subscribe_trade(&self, pairs: &[String]) {
                self.client.subscribe_trade(pairs);
            }
        }
    };
}

impl_trade!(HuobiSpotWSClient);
impl_trade!(HuobiFutureWSClient);
impl_trade!(HuobiInverseSwapWSClient);
impl_trade!(HuobiLinearSwapWSClient);
impl_trade!(HuobiOptionWSClient);

macro_rules! impl_ticker {
    ($struct_name:ident) => {
        impl Ticker for $struct_name {
            fn subscribe_ticker(&self, pairs: &[String]) {
                self.client.subscribe_ticker(pairs);
            }
        }
    };
}

impl_ticker!(HuobiSpotWSClient);
impl_ticker!(HuobiFutureWSClient);
impl_ticker!(HuobiInverseSwapWSClient);
impl_ticker!(HuobiLinearSwapWSClient);
impl_ticker!(HuobiOptionWSClient);

macro_rules! impl_bbo {
    ($struct_name:ident) => {
        impl BBO for $struct_name {
            fn subscribe_bbo(&self, pairs: &[String]) {
                self.client.subscribe_bbo(pairs);
            }
        }
    };
}

impl_bbo!(HuobiSpotWSClient);
impl_bbo!(HuobiFutureWSClient);
impl_bbo!(HuobiInverseSwapWSClient);
impl_bbo!(HuobiLinearSwapWSClient);
impl_bbo!(HuobiOptionWSClient);

macro_rules! impl_orderbook {
    ($struct_name:ident) => {
        impl OrderBook for $struct_name {
            fn subscribe_orderbook(&self, pairs: &[String]) {
                self.client.subscribe_orderbook(pairs);
            }
        }
    };
}

impl_orderbook!(HuobiFutureWSClient);
impl_orderbook!(HuobiInverseSwapWSClient);
impl_orderbook!(HuobiLinearSwapWSClient);
impl_orderbook!(HuobiOptionWSClient);
impl OrderBook for HuobiSpotWSClient {
    fn subscribe_orderbook(&self, pairs: &[String]) {
        if self.client.client.url.as_str() == "wss://api.huobi.pro/feed"
            || self.client.client.url.as_str() == "wss://api-aws.huobi.pro/feed"
        {
            let pair_to_raw_channel = |pair: &String| to_raw_channel("mbp.20", pair);

            let channels = pairs
                .iter()
                .map(pair_to_raw_channel)
                .collect::<Vec<String>>();
            self.client.client.subscribe(&channels);
        } else {
            panic!("Huobi Spot market.$symbol.mbp.$levels must use wss://api.huobi.pro/feed or wss://api-aws.huobi.pro/feed");
        }
    }
}

impl_candlestick!(HuobiSpotWSClient);
impl_candlestick!(HuobiFutureWSClient);
impl_candlestick!(HuobiInverseSwapWSClient);
impl_candlestick!(HuobiLinearSwapWSClient);
impl_candlestick!(HuobiOptionWSClient);

macro_rules! impl_orderbook_snapshot {
    ($struct_name:ident) => {
        impl OrderBookTopK for $struct_name {
            fn subscribe_orderbook_topk(&self, pairs: &[String]) {
                self.client.subscribe_orderbook_topk(pairs);
            }
        }
    };
}

#[rustfmt::skip]
impl_trait!(OrderBookTopK, HuobiSpotWSClient, subscribe_orderbook_topk, "depth.step1", to_raw_channel);
impl_orderbook_snapshot!(HuobiFutureWSClient);
impl_orderbook_snapshot!(HuobiInverseSwapWSClient);
impl_orderbook_snapshot!(HuobiLinearSwapWSClient);
impl_orderbook_snapshot!(HuobiOptionWSClient);

panic_l3_orderbook!(HuobiSpotWSClient);
panic_l3_orderbook!(HuobiFutureWSClient);
panic_l3_orderbook!(HuobiInverseSwapWSClient);
panic_l3_orderbook!(HuobiLinearSwapWSClient);
panic_l3_orderbook!(HuobiOptionWSClient);

#[cfg(test)]
mod tests {
    #[test]
    fn test_channel_to_command() {
        assert_eq!(
            r#"{"sub":"market.btcusdt.trade.detail","id":"crypto-ws-client"}"#,
            super::HuobiWSClient::channel_to_command("market.btcusdt.trade.detail", true)
        );

        assert_eq!(
            r#"{"unsub":"market.btcusdt.trade.detail","id":"crypto-ws-client"}"#,
            super::HuobiWSClient::channel_to_command("market.btcusdt.trade.detail", false)
        );
    }
}
