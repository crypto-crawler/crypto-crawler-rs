use crate::WSClient;
use std::collections::HashMap;

use log::*;
use serde_json::Value;
use tungstenite::Message;

use super::ws_client_internal::{MiscMessage, WSClientInternal};
use super::{OrderBook, OrderBookSnapshot, Ticker, Trade, BBO};

pub(super) const EXCHANGE_NAME: &str = "Huobi";

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
/// * WebSocket API doc: <<https://huobiapi.github.io/docs/coin_margined_swap/v1/en/>>
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
    fn new(url: &str, on_msg: Box<dyn FnMut(String) + 'a>) -> Self {
        HuobiWSClient {
            client: WSClientInternal::new(
                EXCHANGE_NAME,
                url,
                on_msg,
                Self::on_misc_msg,
                Self::channels_to_commands,
            ),
        }
    }

    fn channel_to_command(channel: &str, subscribe: bool) -> String {
        if channel.starts_with('{') {
            return channel.to_string();
        }
        format!(
            r#"{{"{}":"{}","id":"crypto-ws-client"}}"#,
            if subscribe { "sub" } else { "unsub" },
            channel
        )
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
            let value = obj.get("ping").unwrap();
            let mut pong_msg = HashMap::<String, &Value>::new();
            pong_msg.insert("pong".to_string(), value);
            let ws_msg = Message::Text(serde_json::to_string(&pong_msg).unwrap());
            return MiscMessage::WebSocket(ws_msg);
        }

        if let Some(status) = obj.get("status") {
            if status.as_str().unwrap() != "ok" {
                error!("Received {} from {}", msg, EXCHANGE_NAME);
                return MiscMessage::Misc;
            }
        }

        if !obj.contains_key("ch") || !obj.contains_key("ts") {
            warn!("Received {} from {}", msg, EXCHANGE_NAME);
            return MiscMessage::Misc;
        }
        MiscMessage::Normal
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
impl_trait!(OrderBook, HuobiWSClient, subscribe_orderbook, "depth.size_20.high_freq", to_raw_channel);
#[rustfmt::skip]
impl_trait!(OrderBookSnapshot, HuobiWSClient, subscribe_orderbook_snapshot, "depth.step7", to_raw_channel);

/// Define market specific client.
macro_rules! define_market_client {
    ($struct_name:ident, $default_url:ident) => {
        impl<'a> WSClient<'a> for $struct_name<'a> {
            fn new(on_msg: Box<dyn FnMut(String) + 'a>, url: Option<&str>) -> Self {
                let real_url = match url {
                    Some(endpoint) => endpoint,
                    None => $default_url,
                };
                $struct_name {
                    client: HuobiWSClient::new(real_url, on_msg),
                }
            }

            fn subscribe_trade(&mut self, channels: &[String]) {
                <$struct_name as Trade>::subscribe_trade(self, channels);
            }

            fn subscribe_orderbook(&mut self, channels: &[String]) {
                <$struct_name as OrderBook>::subscribe_orderbook(self, channels);
            }

            fn subscribe_ticker(&mut self, channels: &[String]) {
                <$struct_name as Ticker>::subscribe_ticker(self, channels);
            }

            fn subscribe_bbo(&mut self, channels: &[String]) {
                <$struct_name as BBO>::subscribe_bbo(self, channels);
            }

            fn subscribe(&mut self, channels: &[String]) {
                self.client.client.subscribe(channels);
            }

            fn unsubscribe(&mut self, channels: &[String]) {
                self.client.client.unsubscribe(channels);
            }

            fn run(&mut self, duration: Option<u64>) {
                self.client.client.run(duration);
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
            fn subscribe_trade(&mut self, pairs: &[String]) {
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
            fn subscribe_ticker(&mut self, pairs: &[String]) {
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
            fn subscribe_bbo(&mut self, pairs: &[String]) {
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
            fn subscribe_orderbook(&mut self, pairs: &[String]) {
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
    fn subscribe_orderbook(&mut self, pairs: &[String]) {
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
        self.client.subscribe_orderbook(pairs);
    }
}

macro_rules! impl_orderbook_snapshot {
    ($struct_name:ident) => {
        impl<'a> OrderBookSnapshot for $struct_name<'a> {
            fn subscribe_orderbook_snapshot(&mut self, pairs: &[String]) {
                self.client.subscribe_orderbook_snapshot(pairs);
            }
        }
    };
}

impl_orderbook_snapshot!(HuobiFutureWSClient);
impl_orderbook_snapshot!(HuobiInverseSwapWSClient);
impl_orderbook_snapshot!(HuobiLinearSwapWSClient);
impl_orderbook_snapshot!(HuobiOptionWSClient);
impl<'a> OrderBookSnapshot for HuobiSpotWSClient<'a> {
    fn subscribe_orderbook_snapshot(&mut self, pairs: &[String]) {
        let pair_to_raw_channel = |pair: &String| to_raw_channel("depth.step1", pair);

        let channels = pairs
            .iter()
            .map(pair_to_raw_channel)
            .collect::<Vec<String>>();
        self.client.client.subscribe(&channels);
    }
}

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
