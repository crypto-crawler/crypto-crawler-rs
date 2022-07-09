use async_trait::async_trait;
use std::collections::HashMap;

use log::*;
use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;

use crate::{
    common::{
        command_translator::CommandTranslator,
        message_handler::{MessageHandler, MiscMessage},
        ws_client_internal::WSClientInternal,
    },
    WSClient,
};

pub(crate) const EXCHANGE_NAME: &str = "huobi";

// or wss://api-aws.huobi.pro/feed
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
pub struct HuobiWSClient<const URL: char> {
    client: WSClientInternal<HuobiMessageHandler>,
    translator: HuobiCommandTranslator,
}

/// Huobi Spot market.
///
/// * WebSocket API doc: <https://huobiapi.github.io/docs/spot/v1/en/>
/// * Trading at: <https://www.huobi.com/en-us/exchange/>
pub type HuobiSpotWSClient = HuobiWSClient<'S'>;

/// Huobi Future market.
///
/// * WebSocket API doc: <https://huobiapi.github.io/docs/dm/v1/en/>
/// * Trading at: <https://futures.huobi.com/en-us/contract/exchange/>
pub type HuobiFutureWSClient = HuobiWSClient<'F'>;

/// Huobi Inverse Swap market.
///
/// Inverse Swap market uses coins like BTC as collateral.
///
/// * WebSocket API doc: <https://huobiapi.github.io/docs/coin_margined_swap/v1/en/>
/// * Trading at: <https://futures.huobi.com/en-us/swap/exchange/>
pub type HuobiInverseSwapWSClient = HuobiWSClient<'I'>;

/// Huobi Linear Swap market.
///
/// Linear Swap market uses USDT as collateral.
///
/// * WebSocket API doc: <https://huobiapi.github.io/docs/usdt_swap/v1/en/>
/// * Trading at: <https://futures.huobi.com/en-us/linear_swap/exchange/>
pub type HuobiLinearSwapWSClient = HuobiWSClient<'L'>;

/// Huobi Option market.
///
///
/// * WebSocket API doc: <https://huobiapi.github.io/docs/option/v1/en/>
/// * Trading at: <https://futures.huobi.com/en-us/option/exchange/>
pub type HuobiOptionWSClient = HuobiWSClient<'O'>;

impl<const URL: char> HuobiWSClient<URL> {
    pub async fn new(tx: std::sync::mpsc::Sender<String>, url: Option<&str>) -> Self {
        let real_url = match url {
            Some(endpoint) => endpoint,
            None => {
                if URL == 'S' {
                    SPOT_WEBSOCKET_URL
                } else if URL == 'F' {
                    FUTURES_WEBSOCKET_URL
                } else if URL == 'I' {
                    COIN_SWAP_WEBSOCKET_URL
                } else if URL == 'L' {
                    USDT_SWAP_WEBSOCKET_URL
                } else if URL == 'O' {
                    OPTION_WEBSOCKET_URL
                } else {
                    panic!("Unknown URL {}", URL);
                }
            }
        };
        HuobiWSClient {
            client: WSClientInternal::connect(
                EXCHANGE_NAME,
                real_url,
                HuobiMessageHandler {},
                None,
                tx,
            )
            .await,
            translator: HuobiCommandTranslator {},
        }
    }
}

#[async_trait]
impl<const URL: char> WSClient for HuobiWSClient<URL> {
    async fn subscribe_trade(&self, symbols: &[String]) {
        let topics = symbols
            .iter()
            .map(|symbol| ("trade.detail".to_string(), symbol.to_string()))
            .collect::<Vec<(String, String)>>();
        self.subscribe(&topics).await;
    }

    async fn subscribe_orderbook(&self, symbols: &[String]) {
        if URL == 'S' {
            let topics = symbols
                .iter()
                .map(|symbol| ("mbp.20".to_string(), symbol.to_string()))
                .collect::<Vec<(String, String)>>();
            self.subscribe(&topics).await;
        } else {
            let commands = symbols
                .iter()
                .map(|symbol| format!(r#"{{"sub":"market.{}.depth.size_20.high_freq","data_type":"incremental","id": "crypto-ws-client"}}"#, symbol))
                .collect::<Vec<String>>();
            self.client.send(&commands).await;
        }
    }

    async fn subscribe_orderbook_topk(&self, symbols: &[String]) {
        let channel = if URL == 'S' {
            "depth.step1"
        } else {
            "depth.step7"
        };
        let topics = symbols
            .iter()
            .map(|symbol| (channel.to_string(), symbol.to_string()))
            .collect::<Vec<(String, String)>>();
        self.subscribe(&topics).await;
    }

    async fn subscribe_l3_orderbook(&self, _symbols: &[String]) {
        panic!(
            "{} does NOT have the level3 websocket channel",
            EXCHANGE_NAME
        );
    }

    async fn subscribe_ticker(&self, symbols: &[String]) {
        let topics = symbols
            .iter()
            .map(|symbol| ("detail".to_string(), symbol.to_string()))
            .collect::<Vec<(String, String)>>();
        self.subscribe(&topics).await;
    }

    async fn subscribe_bbo(&self, symbols: &[String]) {
        let topics = symbols
            .iter()
            .map(|symbol| ("bbo".to_string(), symbol.to_string()))
            .collect::<Vec<(String, String)>>();
        self.subscribe(&topics).await;
    }

    async fn subscribe_candlestick(&self, symbol_interval_list: &[(String, usize)]) {
        let commands = self
            .translator
            .translate_to_candlestick_commands(true, symbol_interval_list);
        self.client.send(&commands).await;
    }

    async fn subscribe(&self, topics: &[(String, String)]) {
        let commands = self.translator.translate_to_commands(true, topics);
        self.client.send(&commands).await;
    }

    async fn unsubscribe(&self, topics: &[(String, String)]) {
        let commands = self.translator.translate_to_commands(false, topics);
        self.client.send(&commands).await;
    }

    async fn send(&self, commands: &[String]) {
        self.client.send(commands).await;
    }

    async fn run(&self) {
        self.client.run().await;
    }

    fn close(&self) {
        self.client.close();
    }
}

struct HuobiMessageHandler {}
struct HuobiCommandTranslator {}

impl HuobiCommandTranslator {
    fn topic_to_command(channel: &str, symbol: &str, subscribe: bool) -> String {
        let raw_channel = format!("market.{}.{}", symbol, channel);
        format!(
            r#"{{"{}":"{}","id":"crypto-ws-client"}}"#,
            if subscribe { "sub" } else { "unsub" },
            raw_channel
        )
    }

    // see https://huobiapi.github.io/docs/dm/v1/en/#subscribe-kline-data
    fn to_candlestick_raw_channel(interval: usize) -> String {
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
        format!("kline.{}", interval_str)
    }
}

impl MessageHandler for HuobiMessageHandler {
    fn handle_message(&mut self, msg: &str) -> MiscMessage {
        let resp = serde_json::from_str::<HashMap<String, Value>>(msg);
        if resp.is_err() {
            error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
            return MiscMessage::Other;
        }
        let obj = resp.unwrap();

        // Market Heartbeat
        if obj.contains_key("ping") {
            // The server will send a heartbeat every 5 seconds
            // see links in get_ping_msg_and_interval()S
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
                    "sub" | "unsub" => MiscMessage::Other,
                    "notify" => MiscMessage::Normal,
                    _ => {
                        warn!("Received {} from {}", msg, EXCHANGE_NAME);
                        MiscMessage::Other
                    }
                };
            } else {
                warn!("Received {} from {}", msg, EXCHANGE_NAME);
            }
            MiscMessage::Other
        }
    }

    fn get_ping_msg_and_interval(&self) -> Option<(Message, u64)> {
        // The server will send a heartbeat every 5 seconds,
        // - Spot <https://huobiapi.github.io/docs/spot/v1/en/#heartbeat-and-connection>
        // - Future <https://huobiapi.github.io/docs/dm/v1/en/#market-heartbeat>
        // - InverseSwap <https://huobiapi.github.io/docs/coin_margined_swap/v1/en/#market-heartbeat>
        // - LinearSwap <https://huobiapi.github.io/docs/usdt_swap/v1/en/#market-heartbeat>
        // - Option <https://huobiapi.github.io/docs/option/v1/en/#market-heartbeat>
        None
    }
}

impl CommandTranslator for HuobiCommandTranslator {
    fn translate_to_commands(&self, subscribe: bool, topics: &[(String, String)]) -> Vec<String> {
        topics
            .iter()
            .map(|(channel, symbol)| {
                HuobiCommandTranslator::topic_to_command(channel, symbol, subscribe)
            })
            .collect()
    }

    fn translate_to_candlestick_commands(
        &self,
        subscribe: bool,
        symbol_interval_list: &[(String, usize)],
    ) -> Vec<String> {
        let topics = symbol_interval_list
            .iter()
            .map(|(symbol, interval)| {
                let channel = Self::to_candlestick_raw_channel(*interval);
                (channel, symbol.to_string())
            })
            .collect::<Vec<(String, String)>>();
        self.translate_to_commands(subscribe, &topics)
    }
}

#[cfg(test)]
mod tests {
    use crate::common::command_translator::CommandTranslator;

    #[test]
    fn test_one_topic() {
        let translator = super::HuobiCommandTranslator {};
        let commands = translator
            .translate_to_commands(true, &[("trade.detail".to_string(), "btcusdt".to_string())]);

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"sub":"market.btcusdt.trade.detail","id":"crypto-ws-client"}"#,
            commands[0]
        );
    }

    #[test]
    fn test_two_topics() {
        let translator = super::HuobiCommandTranslator {};
        let commands = translator.translate_to_commands(
            true,
            &[
                ("trade.detail".to_string(), "btcusdt".to_string()),
                ("bbo".to_string(), "btcusdt".to_string()),
            ],
        );

        assert_eq!(2, commands.len());
        assert_eq!(
            r#"{"sub":"market.btcusdt.trade.detail","id":"crypto-ws-client"}"#,
            commands[0]
        );
        assert_eq!(
            r#"{"sub":"market.btcusdt.bbo","id":"crypto-ws-client"}"#,
            commands[1]
        );
    }
}
