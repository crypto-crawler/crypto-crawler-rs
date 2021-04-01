use crate::WSClient;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use log::*;
use serde_json::Value;
use tungstenite::Message;

use super::ws_client_internal::{MiscMessage, WSClientInternal};
use super::{Candlestick, OrderBook, OrderBookSnapshot, Ticker, Trade, BBO};

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

// Internal unified client
struct HuobiWSClient<'a> {
    client: WSClientInternal<'a>,
}

/// Huobi Spot market.
///
/// * WebSocket API doc: <https://huobiapi.github.io/docs/spot/v1/en/>
/// * Trading at: <https://www.huobi.com/en-us/exchange/>
pub struct HuobiSpotWSClient<'a> {
    client: HuobiWSClient<'a>,
}

/// Huobi Future market.
///
/// * WebSocket API doc: <https://huobiapi.github.io/docs/dm/v1/en/>
/// * Trading at: <https://futures.huobi.com/en-us/contract/exchange/>
pub struct HuobiFutureWSClient<'a> {
    client: HuobiWSClient<'a>,
}

/// Huobi Inverse Swap market.
///
/// Inverse Swap market uses coins like BTC as collateral.
///
/// * WebSocket API doc: <https://huobiapi.github.io/docs/coin_margined_swap/v1/en/>
/// * Trading at: <https://futures.huobi.com/en-us/swap/exchange/>
pub struct HuobiInverseSwapWSClient<'a> {
    client: HuobiWSClient<'a>,
}

/// Huobi Linear Swap market.
///
/// Linear Swap market uses USDT as collateral.
///
/// * WebSocket API doc: <https://huobiapi.github.io/docs/usdt_swap/v1/en/>
/// * Trading at: <https://futures.huobi.com/en-us/linear_swap/exchange/>
pub struct HuobiLinearSwapWSClient<'a> {
    client: HuobiWSClient<'a>,
}
/// Huobi Option market.
///
///
/// * WebSocket API doc: <https://huobiapi.github.io/docs/option/v1/en/>
/// * Trading at: <https://futures.huobi.com/en-us/option/exchange/>
pub struct HuobiOptionWSClient<'a> {
    client: HuobiWSClient<'a>,
}

impl<'a> HuobiWSClient<'a> {
    fn new(url: &str, on_msg: Arc<Mutex<dyn FnMut(String) + 'a + Send>>) -> Self {
        HuobiWSClient {
            client: WSClientInternal::new(
                EXCHANGE_NAME,
                url,
                on_msg,
                Self::on_misc_msg,
                Self::channels_to_commands,
                None,
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
        let resp = serde_json::from_str::<HashMap<String, Value>>(&msg);
        if resp.is_err() {
            error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
            return MiscMessage::Misc;
        }
        let obj = resp.unwrap();

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
                        if err_msg.starts_with("invalid topic") {
                            panic!("Received {} from {}", msg, EXCHANGE_NAME);
                        }
                    }
                    _ => warn!("Received {} from {}", msg, EXCHANGE_NAME),
                }
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
impl_trait!(OrderBookSnapshot, HuobiWSClient, subscribe_orderbook_snapshot, "depth.step0", to_raw_channel);

impl<'a> OrderBook for HuobiWSClient<'a> {
    fn subscribe_orderbook(&self, pairs: &[String]) {
        let pair_to_raw_channel = |pair: &String| {
            format!(
                r#"{{
            "sub": "market.{}.depth.size_150.high_freq",
            "data_type":"incremental",
            "id": "crypto-ws-client"
        }}"#,
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

fn to_candlestick_raw_channel(pair: &str, interval: u32) -> String {
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
        impl<'a> WSClient<'a> for $struct_name<'a> {
            fn new(on_msg: Arc<Mutex<dyn FnMut(String) + 'a + Send>>, url: Option<&str>) -> Self {
                let real_url = match url {
                    Some(endpoint) => endpoint,
                    None => $default_url,
                };
                $struct_name {
                    client: HuobiWSClient::new(real_url, on_msg),
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
        impl<'a> Trade for $struct_name<'a> {
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
        impl<'a> Ticker for $struct_name<'a> {
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
        impl<'a> BBO for $struct_name<'a> {
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
        impl<'a> OrderBook for $struct_name<'a> {
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
impl<'a> OrderBook for HuobiSpotWSClient<'a> {
    fn subscribe_orderbook(&self, pairs: &[String]) {
        if self.client.client.url.as_str() == "wss://api.huobi.pro/feed"
            || self.client.client.url.as_str() == "wss://api-aws.huobi.pro/feed"
        {
            let pair_to_raw_channel = |pair: &String| to_raw_channel("mbp.150", pair);

            let channels = pairs
                .iter()
                .map(pair_to_raw_channel)
                .collect::<Vec<String>>();
            self.client.client.subscribe(&channels);
        } else {
            panic!("Huobi Spot market.$symbol.mbp.$levels must use wss://api.huobi.pro/feed or wss://api-aws.huobi.pro/feed");
        }
        self.client.subscribe_orderbook(pairs);
    }
}

impl_candlestick!(HuobiSpotWSClient);
impl_candlestick!(HuobiFutureWSClient);
impl_candlestick!(HuobiInverseSwapWSClient);
impl_candlestick!(HuobiLinearSwapWSClient);
impl_candlestick!(HuobiOptionWSClient);

macro_rules! impl_orderbook_snapshot {
    ($struct_name:ident) => {
        impl<'a> OrderBookSnapshot for $struct_name<'a> {
            fn subscribe_orderbook_snapshot(&self, pairs: &[String]) {
                self.client.subscribe_orderbook_snapshot(pairs);
            }
        }
    };
}

impl_orderbook_snapshot!(HuobiSpotWSClient);
impl_orderbook_snapshot!(HuobiFutureWSClient);
impl_orderbook_snapshot!(HuobiInverseSwapWSClient);
impl_orderbook_snapshot!(HuobiLinearSwapWSClient);
impl_orderbook_snapshot!(HuobiOptionWSClient);

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
