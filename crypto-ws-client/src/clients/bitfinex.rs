use async_trait::async_trait;
use std::{collections::HashMap, time::Duration};
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

pub(super) const EXCHANGE_NAME: &str = "bitfinex";

const WEBSOCKET_URL: &str = "wss://api-pub.bitfinex.com/ws/2";

/// The WebSocket client for Bitfinex, including all markets.
///
/// * WebSocket API doc: <https://docs.bitfinex.com/docs/ws-general>
/// * Spot: <https://trading.bitfinex.com/trading>
/// * Swap: <https://trading.bitfinex.com/t/BTCF0:USTF0>
/// * Funding: <https://trading.bitfinex.com/funding>
pub struct BitfinexWSClient {
    client: WSClientInternal<BitfinexMessageHandler>,
    translator: BitfinexCommandTranslator, // used by close() and run()
}

impl_new_constructor!(
    BitfinexWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    BitfinexMessageHandler {
        channel_id_meta: HashMap::new()
    },
    BitfinexCommandTranslator {}
);

impl_trait!(Trade, BitfinexWSClient, subscribe_trade, "trades");
impl_trait!(Ticker, BitfinexWSClient, subscribe_ticker, "ticker");
impl_candlestick!(BitfinexWSClient);

panic_l2_topk!(BitfinexWSClient);

#[async_trait]
impl OrderBook for BitfinexWSClient {
    async fn subscribe_orderbook(&self, symbols: &[String]) {
        let commands = symbols
            .iter()
            .map(|symbol| {
                format!(r#"{{"event": "subscribe","channel": "book","symbol": "{}","prec": "P0","frec": "F0","len":25}}"#,
                    symbol,
                )
            })
            .collect::<Vec<String>>();

        self.send(&commands).await;
    }
}

#[async_trait]
impl BBO for BitfinexWSClient {
    async fn subscribe_bbo(&self, symbols: &[String]) {
        let commands = symbols
            .iter()
            .map(|symbol| {
                format!(
                    r#"{{"event": "subscribe","channel": "book","symbol": "{}","prec": "R0","len": 1}}"#,
                    symbol,
                )
            })
            .collect::<Vec<String>>();

        self.send(&commands).await;
    }
}

#[async_trait]
impl Level3OrderBook for BitfinexWSClient {
    async fn subscribe_l3_orderbook(&self, symbols: &[String]) {
        let commands = symbols
            .iter()
            .map(|symbol| {
                format!(r#"{{"event": "subscribe","channel": "book","symbol": "{}","prec": "R0","len": 250}}"#,
                    symbol,
                )
            })
            .collect::<Vec<String>>();

        self.send(&commands).await;
    }
}

impl_ws_client_trait!(BitfinexWSClient);

struct BitfinexMessageHandler {
    channel_id_meta: HashMap<i64, String>, // CHANNEL_ID information
}
struct BitfinexCommandTranslator {}

impl BitfinexCommandTranslator {
    fn topic_to_command(channel: &str, symbol: &str, subscribe: bool) -> String {
        format!(
            r#"{{"event": "{}", "channel": "{}", "symbol": "{}"}}"#,
            if subscribe {
                "subscribe"
            } else {
                "unsubscribe"
            },
            channel,
            symbol
        )
    }
    fn to_candlestick_command(symbol: &str, interval: usize, subscribe: bool) -> String {
        let interval_str = match interval {
            60 => "1m",
            300 => "5m",
            900 => "15m",
            1800 => "30m",
            3600 => "1h",
            10800 => "3h",
            21600 => "6h",
            43200 => "12h",
            86400 => "1D",
            604800 => "7D",
            1209600 => "14D",
            2592000 => "1M",
            _ => panic!("Bitfinex available intervals 1m,5m,15m,30m,1h,3h,6h,12h,1D,7D,14D,1M"),
        };

        format!(
            r#"{{"event": "{}","channel": "candles","key": "trade:{}:{}"}}"#,
            if subscribe {
                "subscribe"
            } else {
                "unsubscribe"
            },
            interval_str,
            symbol
        )
    }
}

impl MessageHandler for BitfinexMessageHandler {
    fn handle_message(&mut self, txt: &str) -> MiscMessage {
        if txt.starts_with('{') {
            let mut obj = serde_json::from_str::<HashMap<String, Value>>(txt).unwrap();
            let event = obj.get("event").unwrap().as_str().unwrap();
            match event {
                "error" => {
                    let code = obj.get("code").unwrap().as_i64().unwrap();
                    match code {
                        10301 | 10401 => {
                            // 10301: Already subscribed
                            // 10401: Not subscribed
                            // 10000: Unknown event
                            warn!("{} from {}", txt, EXCHANGE_NAME);
                        }
                        10300 | 10400 | 10302 => {
                            // 10300, 10400:Subscription failed
                            // 10302: Unknown channel
                            // 10001: Unknown pair
                            // 10305: Reached limit of open channels
                            error!("{} from {}", txt, EXCHANGE_NAME);
                            panic!("{} from {}", txt, EXCHANGE_NAME);
                        }
                        _ => warn!("{} from {}", txt, EXCHANGE_NAME),
                    }
                    MiscMessage::Other
                }
                "info" => {
                    if obj.get("version").is_some() {
                        // 1 for operative, 0 for maintenance
                        let status = obj
                            .get("platform")
                            .unwrap()
                            .as_object()
                            .unwrap()
                            .get("status")
                            .unwrap()
                            .as_i64()
                            .unwrap();
                        if status == 0 {
                            std::thread::sleep(Duration::from_secs(15));
                            MiscMessage::Reconnect
                        } else {
                            MiscMessage::Other
                        }
                    } else {
                        let code = obj.get("code").unwrap().as_i64().unwrap();
                        match code {
                            20051 => {
                                // Stop/Restart Websocket Server (please reconnect)
                                // self.reconnect();
                                error!("Stop/Restart Websocket Server, exiting now...");
                                MiscMessage::Reconnect // fail fast, pm2 will restart
                            }
                            20060 => {
                                // Entering in Maintenance mode. Please pause any activity and resume
                                // after receiving the info message 20061 (it should take 120 seconds
                                // at most).
                                std::thread::sleep(Duration::from_secs(15));
                                MiscMessage::Other
                            }
                            20061 => {
                                // Maintenance ended. You can resume normal activity. It is advised
                                // to unsubscribe/subscribe again all channels.
                                MiscMessage::Reconnect
                            }
                            _ => {
                                info!("{} from {}", txt, EXCHANGE_NAME);
                                MiscMessage::Other
                            }
                        }
                    }
                }
                "pong" => {
                    debug!("{} from {}", txt, EXCHANGE_NAME);
                    MiscMessage::Pong
                }
                "conf" => {
                    warn!("{} from {}", txt, EXCHANGE_NAME);
                    MiscMessage::Other
                }
                "subscribed" => {
                    let chan_id = obj.get("chanId").unwrap().as_i64().unwrap();
                    obj.remove("event");
                    obj.remove("chanId");
                    obj.remove("pair");
                    self.channel_id_meta
                        .insert(chan_id, serde_json::to_string(&obj).unwrap());
                    MiscMessage::Other
                }
                "unsubscribed" => {
                    let chan_id = obj.get("chanId").unwrap().as_i64().unwrap();
                    self.channel_id_meta.remove(&chan_id);
                    MiscMessage::Other
                }
                _ => MiscMessage::Other,
            }
        } else {
            debug_assert!(txt.starts_with('['));
            let arr = serde_json::from_str::<Vec<Value>>(txt).unwrap();
            if arr.is_empty() {
                MiscMessage::Other // ignore empty array
            } else if arr.len() == 2 && arr[1].as_str().unwrap_or("null") == "hb" {
                // If there is no activity in the channel for 15 seconds, the Websocket server
                // will send you a heartbeat message in this format.
                // see <https://docs.bitfinex.com/docs/ws-general#heartbeating>
                MiscMessage::WebSocket(Message::Text(r#"{"event":"ping"}"#.to_string()))
            } else {
                // replace CHANNEL_ID with meta info
                let i = txt.find(',').unwrap(); // first comma, for example, te, tu, see https://blog.bitfinex.com/api/websocket-api-update/
                let channel_id = (txt[1..i]).parse::<i64>().unwrap();
                if let Some(channel_info) = self.channel_id_meta.get(&channel_id) {
                    let new_txt = format!("[{}{}", channel_info, &txt[i..]);
                    MiscMessage::Mutated(new_txt)
                } else {
                    MiscMessage::Other
                }
            }
        }
    }

    fn get_ping_msg_and_interval(&self) -> Option<(Message, u64)> {
        // If there is no activity in the channel for 15 seconds, the Websocket server will send
        // you a heartbeat message, see https://docs.bitfinex.com/docs/ws-general#heartbeating
        None
    }
}

impl CommandTranslator for BitfinexCommandTranslator {
    fn translate_to_commands(&self, subscribe: bool, topics: &[(String, String)]) -> Vec<String> {
        topics
            .iter()
            .map(|(channel, symbol)| Self::topic_to_command(channel, symbol, subscribe))
            .collect()
    }

    fn translate_to_candlestick_commands(
        &self,
        subscribe: bool,
        symbol_interval_list: &[(String, usize)],
    ) -> Vec<String> {
        symbol_interval_list
            .iter()
            .map(|(symbol, interval)| Self::to_candlestick_command(symbol, *interval, subscribe))
            .collect::<Vec<String>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::common::command_translator::CommandTranslator;

    #[test]
    fn test_spot_command() {
        let translator = super::BitfinexCommandTranslator {};
        let commands = translator
            .translate_to_commands(true, &vec![("trades".to_string(), "tBTCUSD".to_string())]);

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"event": "subscribe", "channel": "trades", "symbol": "tBTCUSD"}"#,
            commands[0]
        );
    }

    #[test]
    fn test_swap_command() {
        let translator = super::BitfinexCommandTranslator {};
        let commands = translator.translate_to_commands(
            true,
            &vec![("trades".to_string(), "tBTCF0:USTF0".to_string())],
        );

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"event": "subscribe", "channel": "trades", "symbol": "tBTCF0:USTF0"}"#,
            commands[0]
        );
    }
}
