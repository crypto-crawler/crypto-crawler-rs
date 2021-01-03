use crate::WSClient;
use std::collections::HashMap;

use super::{
    ws_client_internal::{MiscMessage, WSClientInternal},
    Ticker, Trade, BBO,
};
use log::*;
use serde_json::Value;

pub(super) const EXCHANGE_NAME: &str = "Binance";

const SPOT_WEBSOCKET_URL: &str = "wss://stream.binance.com:9443/stream";
const FUTURES_WEBSOCKET_URL: &str = "wss://fstream.binance.com/stream";
const DELIVERY_WEBSOCKET_URL: &str = "wss://dstream.binance.com/stream";

// Internal unified client
struct BinanceWSClient<'a> {
    client: WSClientInternal<'a>,
}

impl<'a> BinanceWSClient<'a> {
    fn new(url: &str, on_msg: Box<dyn FnMut(String) + 'a>) -> Self {
        BinanceWSClient {
            client: WSClientInternal::new(
                EXCHANGE_NAME,
                url,
                on_msg,
                Self::on_misc_msg,
                Self::channels_to_commands,
            ),
        }
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
}


fn to_raw_channel(channel: &str, pair: &str) -> String {
    format!("{}@{}", pair, channel)
}

#[rustfmt::skip]
impl_trait!(Trade, BinanceWSClient, subscribe_trade, "aggTrade", to_raw_channel);
#[rustfmt::skip]
impl_trait!(Ticker, BinanceWSClient, subscribe_ticker, "ticker", to_raw_channel);
#[rustfmt::skip]
impl_trait!(BBO, BinanceWSClient, subscribe_bbo, "bookTicker", to_raw_channel);

/// Binance Spot market.
///
///   * WebSocket API doc: <https://binance-docs.github.io/apidocs/spot/en/>
///   * Trading at: <https://www.binance.com/en/trade/BTC_USDT>
pub struct BinanceSpotWSClient<'a> {
    client: BinanceWSClient<'a>,
}

/// Binance Coin-margined Future market.
///
///   * WebSocket API doc: <https://binance-docs.github.io/apidocs/delivery/en/>
///   * Trading at: <https://www.binance.com/en/delivery/btcusd_quarter>
pub struct BinanceFutureWSClient<'a> {
    client: BinanceWSClient<'a>,
}

/// Binance USDT-margined Perpetual Swap market.
///
///   * WebSocket API doc: <https://binance-docs.github.io/apidocs/futures/en/>
///   * Trading at: <https://www.binance.com/en/futures/BTC_USDT>
pub struct BinanceLinearSwapWSClient<'a> {
    client: BinanceWSClient<'a>,
}

/// Binance Coin-margined Perpetual Swap market
///
///   * WebSocket API doc: <https://binance-docs.github.io/apidocs/delivery/en/>
///   * Trading at: <https://www.binance.com/en/delivery/btcusd_perpetual>
pub struct BinanceInverseSwapWSClient<'a> {
    client: BinanceWSClient<'a>,
}

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
                    client: BinanceWSClient::new(real_url, on_msg),
                }
            }

            fn subscribe_trade(&mut self, channels: &[String]) {
                <$struct_name as Trade>::subscribe_trade(self, channels);
            }

            fn subscribe_ticker(&mut self, channels: &[String]) {
                <$struct_name as Ticker>::subscribe_ticker(self, channels);
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

define_market_client!(BinanceSpotWSClient, SPOT_WEBSOCKET_URL);
define_market_client!(BinanceFutureWSClient, DELIVERY_WEBSOCKET_URL);
define_market_client!(BinanceLinearSwapWSClient, FUTURES_WEBSOCKET_URL);
define_market_client!(BinanceInverseSwapWSClient, DELIVERY_WEBSOCKET_URL);

macro_rules! impl_trade {
    ($struct_name:ident) => {
        impl<'a> Trade for $struct_name<'a> {
            fn subscribe_trade(&mut self, pairs: &[String]) {
                self.client.subscribe_trade(pairs);
            }
        }
    };
}

impl_trade!(BinanceSpotWSClient);
impl_trade!(BinanceFutureWSClient);
impl_trade!(BinanceLinearSwapWSClient);
impl_trade!(BinanceInverseSwapWSClient);

macro_rules! impl_ticker {
    ($struct_name:ident) => {
        impl<'a> Ticker for $struct_name<'a> {
            fn subscribe_ticker(&mut self, pairs: &[String]) {
                self.client.subscribe_ticker(pairs);
            }
        }
    };
}

impl_ticker!(BinanceSpotWSClient);
impl_ticker!(BinanceFutureWSClient);
impl_ticker!(BinanceLinearSwapWSClient);
impl_ticker!(BinanceInverseSwapWSClient);

macro_rules! impl_bbo {
    ($struct_name:ident) => {
        impl<'a> BBO for $struct_name<'a> {
            fn subscribe_bbo(&mut self, pairs: &[String]) {
                self.client.subscribe_bbo(pairs);
            }
        }
    };
}

impl_bbo!(BinanceSpotWSClient);
impl_bbo!(BinanceFutureWSClient);
impl_bbo!(BinanceLinearSwapWSClient);
impl_bbo!(BinanceInverseSwapWSClient);

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
