use async_trait::async_trait;
use nonzero_ext::nonzero;
use std::{
    collections::{BTreeMap, HashMap},
    num::NonZeroU32,
};
use tokio_tungstenite::tungstenite::Message;

use log::*;
use serde_json::Value;

use crate::{
    clients::common_traits::{
        Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO,
    },
    common::{
        command_translator::CommandTranslator,
        message_handler::{MessageHandler, MiscMessage},
        utils::ensure_frame_size,
        ws_client_internal::WSClientInternal,
    },
    WSClient,
};

pub(crate) const EXCHANGE_NAME: &str = "okx";

const WEBSOCKET_URL: &str = "wss://ws.okx.com:8443/ws/v5/public";

/// https://www.okx.com/docs-v5/en/#websocket-api-subscribe
/// The total length of multiple channels cannot exceed 4096 bytes
const WS_FRAME_SIZE: usize = 4096;

// Subscription limit: 240 times per hour
// see https://www.okx.com/docs-v5/en/#websocket-api-connect
const UPLINK_LIMIT: (NonZeroU32, std::time::Duration) =
    (nonzero!(240u32), std::time::Duration::from_secs(3600));

/// The WebSocket client for OKX.
///
/// OKX has Spot, Future, Swap and Option markets.
///
/// * API doc: <https://www.okx.com/docs-v5/en/#websocket-api>
/// * Trading at:
///     * Spot <https://www.okx.com/trade-spot>
///     * Future <https://www.okx.com/trade-futures>
///     * Swap <https://www.okx.com/trade-swap>
///     * Option <https://www.okx.com/trade-option>
pub struct OkxWSClient {
    client: WSClientInternal<OkxMessageHandler>,
    translator: OkxCommandTranslator,
}

impl OkxWSClient {
    pub async fn new(tx: std::sync::mpsc::Sender<String>, url: Option<&str>) -> Self {
        let real_url = match url {
            Some(endpoint) => endpoint,
            None => WEBSOCKET_URL,
        };
        OkxWSClient {
            client: WSClientInternal::connect(
                EXCHANGE_NAME,
                real_url,
                OkxMessageHandler {},
                Some(UPLINK_LIMIT),
                tx,
            )
            .await,
            translator: OkxCommandTranslator {},
        }
    }
}

impl_trait!(Trade, OkxWSClient, subscribe_trade, "trades");
impl_trait!(Ticker, OkxWSClient, subscribe_ticker, "tickers");
impl_trait!(BBO, OkxWSClient, subscribe_bbo, "bbo-tbt");
#[rustfmt::skip]
// books-l2-tbt and books50-l2-tbt require login, only books doesn't require it
impl_trait!(OrderBook, OkxWSClient, subscribe_orderbook, "books");
#[rustfmt::skip]
impl_trait!(OrderBookTopK, OkxWSClient, subscribe_orderbook_topk, "books5");
impl_candlestick!(OkxWSClient);
panic_l3_orderbook!(OkxWSClient);

impl_ws_client_trait!(OkxWSClient);

struct OkxMessageHandler {}
struct OkxCommandTranslator {}

impl OkxCommandTranslator {
    fn topics_to_command(chunk: &[(String, String)], subscribe: bool) -> String {
        let arr = chunk
            .iter()
            .map(|t| {
                let mut map = BTreeMap::new();
                let (channel, symbol) = t;
                map.insert("channel".to_string(), channel.to_string());
                map.insert("instId".to_string(), symbol.to_string());
                map
            })
            .collect::<Vec<BTreeMap<String, String>>>();
        format!(
            r#"{{"op":"{}","args":{}}}"#,
            if subscribe {
                "subscribe"
            } else {
                "unsubscribe"
            },
            serde_json::to_string(&arr).unwrap(),
        )
    }

    // see https://www.okx.com/docs-v5/en/#websocket-api-public-channel-candlesticks-channel
    fn to_candlestick_raw_channel(interval: usize) -> &'static str {
        match interval {
            60 => "candle1m",
            180 => "candle3m",
            300 => "candle5m",
            900 => "candle15m",
            1800 => "candle30m",
            3600 => "candle1H",
            7200 => "candle2H",
            14400 => "candle4H",
            21600 => "candle6H",
            43200 => "candle12H",
            86400 => "candle1D",
            172800 => "candle2D",
            259200 => "candle3D",
            432000 => "candle5D",
            604800 => "candle1W",
            2592000 => "candle1M",
            _ => panic!("Invalid OKX candlestick interval {}", interval),
        }
    }
}

impl MessageHandler for OkxMessageHandler {
    fn handle_message(&mut self, msg: &str) -> MiscMessage {
        if msg == "pong" {
            return MiscMessage::Pong;
        }
        let resp = serde_json::from_str::<HashMap<String, Value>>(msg);
        if resp.is_err() {
            error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
            return MiscMessage::Other;
        }
        let obj = resp.unwrap();

        if let Some(event) = obj.get("event") {
            match event.as_str().unwrap() {
                "error" => {
                    let error_code = obj
                        .get("code")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .parse::<i64>()
                        .unwrap();
                    match error_code {
                        30040 => {
                            // channel doesn't exist, ignore because some symbols don't exist in websocket while they exist in `/v3/instruments`
                            error!("Received {} from {}", msg, EXCHANGE_NAME);
                        }
                        _ => panic!("Received {} from {}", msg, EXCHANGE_NAME),
                    }
                }
                "subscribe" => info!("Received {} from {}", msg, EXCHANGE_NAME),
                "unsubscribe" => info!("Received {} from {}", msg, EXCHANGE_NAME),
                _ => warn!("Received {} from {}", msg, EXCHANGE_NAME),
            }
            MiscMessage::Other
        } else if !obj.contains_key("arg") || !obj.contains_key("data") {
            error!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Other
        } else {
            MiscMessage::Normal
        }
    }

    fn get_ping_msg_and_interval(&self) -> Option<(Message, u64)> {
        // https://www.okx.com/docs-v5/en/#websocket-api-connect
        Some((Message::Text("ping".to_string()), 30))
    }
}

impl CommandTranslator for OkxCommandTranslator {
    fn translate_to_commands(&self, subscribe: bool, topics: &[(String, String)]) -> Vec<String> {
        ensure_frame_size(
            topics,
            subscribe,
            Self::topics_to_command,
            WS_FRAME_SIZE,
            None,
        )
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
                (channel.to_string(), symbol.to_string())
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
        let translator = super::OkxCommandTranslator {};
        let commands = translator
            .translate_to_commands(true, &vec![("trades".to_string(), "BTC-USDT".to_string())]);

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"op":"subscribe","args":[{"channel":"trades","instId":"BTC-USDT"}]}"#,
            commands[0]
        );
    }

    #[test]
    fn test_two_topics() {
        let translator = super::OkxCommandTranslator {};
        let commands = translator.translate_to_commands(
            true,
            &vec![
                ("trades".to_string(), "BTC-USDT".to_string()),
                ("tickers".to_string(), "BTC-USDT".to_string()),
            ],
        );

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"op":"subscribe","args":[{"channel":"trades","instId":"BTC-USDT"},{"channel":"tickers","instId":"BTC-USDT"}]}"#,
            commands[0]
        );
    }
}
