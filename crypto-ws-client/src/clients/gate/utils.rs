use std::collections::HashMap;

use log::*;
use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;

use crate::common::{
    command_translator::CommandTranslator,
    message_handler::{MessageHandler, MiscMessage},
};

pub(super) const EXCHANGE_NAME: &str = "gate";

// MARKET_TYPE: 'S' for spot, 'F' for futures
pub(super) struct GateMessageHandler<const MARKET_TYPE: char> {}
pub(super) struct GateCommandTranslator<const MARKET_TYPE: char> {}

impl<const MARKET_TYPE: char> MessageHandler for GateMessageHandler<MARKET_TYPE> {
    fn handle_message(&mut self, msg: &str) -> MiscMessage {
        let obj = serde_json::from_str::<HashMap<String, Value>>(msg).unwrap();

        // https://www.gate.io/docs/apiv4/ws/en/#server-response
        // Null if the server accepts the client request; otherwise, the detailed reason why request is rejected.
        let error = match obj.get("error") {
            None => serde_json::Value::Null,
            Some(err) => {
                if err.is_null() {
                    serde_json::Value::Null
                } else {
                    err.clone()
                }
            }
        };
        if !error.is_null() {
            let err = error.as_object().unwrap();
            // https://www.gate.io/docs/apiv4/ws/en/#schema_error
            // https://www.gate.io/docs/futures/ws/en/#error
            let code = err.get("code").unwrap().as_i64().unwrap();
            match code {
                1 | 2 => panic!("Received {} from {}", msg, EXCHANGE_NAME), // client side errors
                _ => error!("Received {} from {}", msg, EXCHANGE_NAME),     // server side errors
            }
            return MiscMessage::Other;
        }

        let channel = obj.get("channel").unwrap().as_str().unwrap();
        let event = obj.get("event").unwrap().as_str().unwrap();

        if channel == "spot.pong" || channel == "futures.pong" {
            MiscMessage::Pong
        } else if event == "update" || event == "all" {
            MiscMessage::Normal
        } else if event == "subscribe" || event == "unsubscribe" {
            debug!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Other
        } else {
            warn!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Other
        }
    }

    fn get_ping_msg_and_interval(&self) -> Option<(Message, u64)> {
        if MARKET_TYPE == 'S' {
            // https://www.gate.io/docs/apiv4/ws/en/#application-ping-pong
            Some((Message::Text(r#"{"channel":"spot.ping"}"#.to_string()), 60))
        } else {
            // https://www.gate.io/docs/futures/ws/en/#ping-and-pong
            // https://www.gate.io/docs/delivery/ws/en/#ping-and-pong
            Some((
                Message::Text(r#"{"channel":"futures.ping"}"#.to_string()),
                60,
            ))
        }
    }
}

impl<const MARKET_TYPE: char> GateCommandTranslator<MARKET_TYPE> {
    fn channel_symbols_to_command(
        channel: &str,
        symbols: &[String],
        subscribe: bool,
    ) -> Vec<String> {
        let channel = if MARKET_TYPE == 'S' {
            format!("spot.{}", channel)
        } else if MARKET_TYPE == 'F' {
            format!("futures.{}", channel)
        } else {
            panic!("unexpected market type: {}", MARKET_TYPE)
        };
        if channel.contains(".order_book") {
            symbols
                .iter()
                .map(|symbol| {
                    format!(
                        r#"{{"channel":"{}", "event":"{}", "payload":{}}}"#,
                        channel,
                        if subscribe {
                            "subscribe"
                        } else {
                            "unsubscribe"
                        },
                        if channel.ends_with(".order_book") {
                            if MARKET_TYPE == 'S' {
                                serde_json::to_string(&[symbol, "20", "1000ms"]).unwrap()
                            } else if MARKET_TYPE == 'F' {
                                serde_json::to_string(&[symbol, "20", "0"]).unwrap()
                            } else {
                                panic!("unexpected market type: {}", MARKET_TYPE)
                            }
                        } else if channel.ends_with(".order_book_update") {
                            if MARKET_TYPE == 'S' {
                                serde_json::to_string(&[symbol, "100ms"]).unwrap()
                            } else if MARKET_TYPE == 'F' {
                                serde_json::to_string(&[symbol, "100ms", "20"]).unwrap()
                            } else {
                                panic!("unexpected market type: {}", MARKET_TYPE)
                            }
                        } else {
                            panic!("unexpected channel: {}", channel)
                        },
                    )
                })
                .collect()
        } else {
            vec![format!(
                r#"{{"channel":"{}", "event":"{}", "payload":{}}}"#,
                channel,
                if subscribe {
                    "subscribe"
                } else {
                    "unsubscribe"
                },
                serde_json::to_string(&symbols).unwrap(),
            )]
        }
    }

    fn to_candlestick_command(symbol: &str, interval: usize, subscribe: bool) -> String {
        let interval_str = match interval {
            10 => "10s",
            60 => "1m",
            300 => "5m",
            900 => "15m",
            1800 => "30m",
            3600 => "1h",
            14400 => "4h",
            28800 => "8h",
            86400 => "1d",
            604800 => "7d",
            _ => panic!("Gate available intervals 10s,1m,5m,15m,30m,1h,4h,8h,1d,7d"),
        };
        format!(
            r#"{{"channel": "{}.candlesticks", "event": "{}", "payload" : ["{}", "{}"]}}"#,
            if MARKET_TYPE == 'S' {
                "spot"
            } else {
                "futures"
            },
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

impl<const MARKET_TYPE: char> CommandTranslator for GateCommandTranslator<MARKET_TYPE> {
    fn translate_to_commands(&self, subscribe: bool, topics: &[(String, String)]) -> Vec<String> {
        let mut commands: Vec<String> = Vec::new();

        let mut channel_symbols = HashMap::<String, Vec<String>>::new();
        for (channel, symbol) in topics {
            match channel_symbols.get_mut(channel) {
                Some(symbols) => symbols.push(symbol.to_string()),
                None => {
                    channel_symbols.insert(channel.to_string(), vec![symbol.to_string()]);
                }
            }
        }

        for (channel, symbols) in channel_symbols.iter() {
            commands.extend(Self::channel_symbols_to_command(
                channel, symbols, subscribe,
            ));
        }

        commands
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
    fn test_spot() {
        let translator = super::GateCommandTranslator::<'S'> {};

        assert_eq!(
            r#"{"channel":"spot.trades", "event":"subscribe", "payload":["BTC_USDT","ETH_USDT"]}"#,
            translator.translate_to_commands(
                true,
                &[("trades".to_string(), "BTC_USDT".to_string()),
                    ("trades".to_string(), "ETH_USDT".to_string())]
            )[0]
        );

        let commands = translator.translate_to_commands(
            true,
            &[("order_book".to_string(), "BTC_USDT".to_string()),
                ("order_book".to_string(), "ETH_USDT".to_string())],
        );
        assert_eq!(2, commands.len());
        assert_eq!(
            r#"{"channel":"spot.order_book", "event":"subscribe", "payload":["BTC_USDT","20","1000ms"]}"#,
            commands[0]
        );
        assert_eq!(
            r#"{"channel":"spot.order_book", "event":"subscribe", "payload":["ETH_USDT","20","1000ms"]}"#,
            commands[1]
        );

        let commands = translator.translate_to_commands(
            true,
            &[("order_book_update".to_string(), "BTC_USDT".to_string()),
                ("order_book_update".to_string(), "ETH_USDT".to_string())],
        );
        assert_eq!(2, commands.len());
        assert_eq!(
            r#"{"channel":"spot.order_book_update", "event":"subscribe", "payload":["BTC_USDT","100ms"]}"#,
            commands[0]
        );
        assert_eq!(
            r#"{"channel":"spot.order_book_update", "event":"subscribe", "payload":["ETH_USDT","100ms"]}"#,
            commands[1]
        );
    }

    #[test]
    fn test_futures() {
        let translator = super::GateCommandTranslator::<'F'> {};

        assert_eq!(
            r#"{"channel":"futures.trades", "event":"subscribe", "payload":["BTC_USD","ETH_USD"]}"#,
            translator.translate_to_commands(
                true,
                &[("trades".to_string(), "BTC_USD".to_string()),
                    ("trades".to_string(), "ETH_USD".to_string())]
            )[0]
        );

        let commands = translator.translate_to_commands(
            true,
            &[("order_book".to_string(), "BTC_USD".to_string()),
                ("order_book".to_string(), "ETH_USD".to_string())],
        );
        assert_eq!(2, commands.len());
        assert_eq!(
            r#"{"channel":"futures.order_book", "event":"subscribe", "payload":["BTC_USD","20","0"]}"#,
            commands[0]
        );
        assert_eq!(
            r#"{"channel":"futures.order_book", "event":"subscribe", "payload":["ETH_USD","20","0"]}"#,
            commands[1]
        );

        let commands = translator.translate_to_commands(
            true,
            &[("order_book_update".to_string(), "BTC_USD".to_string()),
                ("order_book_update".to_string(), "ETH_USD".to_string())],
        );
        assert_eq!(2, commands.len());
        assert_eq!(
            r#"{"channel":"futures.order_book_update", "event":"subscribe", "payload":["BTC_USD","100ms","20"]}"#,
            commands[0]
        );
        assert_eq!(
            r#"{"channel":"futures.order_book_update", "event":"subscribe", "payload":["ETH_USD","100ms","20"]}"#,
            commands[1]
        );
    }
}
