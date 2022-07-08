use async_trait::async_trait;
use std::collections::{BTreeMap, HashMap};
use tokio_tungstenite::tungstenite::Message;

use crate::{
    clients::common_traits::{
        Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO,
    },
    common::{
        command_translator::CommandTranslator,
        message_handler::{MessageHandler, MiscMessage},
        ws_client_internal::WSClientInternal,
    },
    WSClient,
};
use log::*;
use serde_json::Value;

pub(super) const EXCHANGE_NAME: &str = "coinbase_pro";

const WEBSOCKET_URL: &str = "wss://ws-feed.exchange.coinbase.com";

/// The WebSocket client for CoinbasePro.
///
/// CoinbasePro has only Spot market.
///
///   * WebSocket API doc: <https://docs.cloud.coinbase.com/exchange/docs/websocket-overview>
///   * Trading at: <https://pro.coinbase.com/>
pub struct CoinbaseProWSClient {
    client: WSClientInternal<CoinbaseProMessageHandler>,
    translator: CoinbaseProCommandTranslator,
}

impl_new_constructor!(
    CoinbaseProWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    CoinbaseProMessageHandler {},
    CoinbaseProCommandTranslator {}
);

impl_trait!(Trade, CoinbaseProWSClient, subscribe_trade, "matches");
impl_trait!(Ticker, CoinbaseProWSClient, subscribe_ticker, "ticker");
#[rustfmt::skip]
impl_trait!(OrderBook, CoinbaseProWSClient, subscribe_orderbook, "level2");
#[rustfmt::skip]
impl_trait!(Level3OrderBook, CoinbaseProWSClient, subscribe_l3_orderbook, "full");

panic_bbo!(CoinbaseProWSClient);
panic_candlestick!(CoinbaseProWSClient);
panic_l2_topk!(CoinbaseProWSClient);

impl_ws_client_trait!(CoinbaseProWSClient);

struct CoinbaseProMessageHandler {}
struct CoinbaseProCommandTranslator {}

impl MessageHandler for CoinbaseProMessageHandler {
    fn handle_message(&mut self, msg: &str) -> MiscMessage {
        let resp = serde_json::from_str::<HashMap<String, Value>>(msg);
        if resp.is_err() {
            error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
            return MiscMessage::Other;
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
                    MiscMessage::Other
                }
            }
            "subscriptions" => {
                info!("Received {} from {}", msg, EXCHANGE_NAME);
                MiscMessage::Other
            }
            "heartbeat" => {
                debug!("Received {} from {}", msg, EXCHANGE_NAME);
                MiscMessage::Other
            }
            _ => MiscMessage::Normal,
        }
    }

    fn get_ping_msg_and_interval(&self) -> Option<(Message, u64)> {
        None
    }
}

impl CommandTranslator for CoinbaseProCommandTranslator {
    fn translate_to_commands(&self, subscribe: bool, topics: &[(String, String)]) -> Vec<String> {
        let mut commands: Vec<String> = Vec::new();

        let mut channel_symbols = BTreeMap::<String, Vec<String>>::new();
        for (channel, symbol) in topics {
            match channel_symbols.get_mut(channel) {
                Some(symbols) => symbols.push(symbol.to_string()),
                None => {
                    channel_symbols.insert(channel.to_string(), vec![symbol.to_string()]);
                }
            }
        }

        if !channel_symbols.is_empty() {
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
            for (channel, symbols) in channel_symbols.iter() {
                command.push_str(
                    format!(
                        r#"{{"name":"{}","product_ids":{}}}"#,
                        channel,
                        serde_json::to_string(symbols).unwrap(),
                    )
                    .as_str(),
                );
                command.push(',')
            }
            command.pop();
            command.push_str("]}");

            commands.push(command);
        }

        commands
    }

    fn translate_to_candlestick_commands(
        &self,
        _subscribe: bool,
        _symbol_interval_list: &[(String, usize)],
    ) -> Vec<String> {
        panic!("CoinbasePro does NOT have candlestick channel");
    }
}

#[cfg(test)]
mod tests {
    use crate::common::command_translator::CommandTranslator;

    #[test]
    fn test_two_symbols() {
        let translator = super::CoinbaseProCommandTranslator {};
        let commands = translator.translate_to_commands(
            true,
            &vec![
                ("matches".to_string(), "BTC-USD".to_string()),
                ("matches".to_string(), "ETH-USD".to_string()),
            ],
        );

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"type":"subscribe","channels": [{"name":"matches","product_ids":["BTC-USD","ETH-USD"]}]}"#,
            commands[0]
        );
    }

    #[test]
    fn test_two_channels() {
        let translator = super::CoinbaseProCommandTranslator {};
        let commands = translator.translate_to_commands(
            true,
            &vec![
                ("matches".to_string(), "BTC-USD".to_string()),
                ("level2".to_string(), "BTC-USD".to_string()),
            ],
        );

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"type":"subscribe","channels": [{"name":"level2","product_ids":["BTC-USD"]},{"name":"matches","product_ids":["BTC-USD"]}]}"#,
            commands[0]
        );
    }
}
