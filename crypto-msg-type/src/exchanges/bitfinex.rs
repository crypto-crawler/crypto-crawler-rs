use std::collections::HashMap;

use crate::MessageType;

fn msg_type_symbol_to_command(
    msg_type: MessageType,
    symbol: &str,
    subscribe: bool,
    configs: Option<&HashMap<String, String>>,
) -> String {
    let sub_or_unsub = if subscribe {
        "subscribe"
    } else {
        "unsubscribe"
    };
    match msg_type {
        MessageType::Trade | MessageType::Ticker => {
            let channel = if msg_type == MessageType::Trade {
                "trades"
            } else {
                "ticker"
            };
            format!(
                r#"{{"event":"{}", "channel":"{}", "symbol":"{}"}}"#,
                sub_or_unsub, channel, symbol
            )
        }
        MessageType::L2Event => format!(
            r#"{{"event":"{}", "channel":"book", "symbol":"{}", "prec":"P0", "frec":"F0", "len":25}}"#,
            sub_or_unsub, symbol
        ),
        MessageType::L3Event | MessageType::BBO => format!(
            r#"{{"event":"{}", "channel":"book", "symbol": "{}", "prec":"R0", "len": {}}}"#,
            sub_or_unsub,
            symbol,
            if msg_type == MessageType::L3Event {
                25
            } else {
                1
            }
        ),
        MessageType::Candlestick => format!(
            r#"{{"event":"{}", "channel":"candles", "key":"trade:{}:{}"}}"#,
            sub_or_unsub,
            configs.unwrap().get("interval").unwrap(),
            symbol
        ),
        _ => panic!("Unknown message type {}", msg_type),
    }
}

pub(crate) fn get_ws_commands(
    msg_types: &[MessageType],
    symbols: &[String],
    subscribe: bool,
    configs: Option<&HashMap<String, String>>,
) -> Vec<String> {
    msg_types
        .iter()
        .flat_map(|msg_type| {
            symbols
                .iter()
                .map(|symbol| msg_type_symbol_to_command(*msg_type, symbol, subscribe, configs))
        })
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_msg_type_multiple_symbols() {
        let commands = get_ws_commands(
            &vec![MessageType::Trade],
            &vec!["tBTCUST".to_string(), "tETHUST".to_string()],
            true,
            None,
        );
        assert_eq!(commands.len(), 2);
        assert_eq!(
            r#"{"event":"subscribe", "channel":"trades", "symbol":"tBTCUST"}"#,
            commands[0]
        );
        assert_eq!(
            r#"{"event":"subscribe", "channel":"trades", "symbol":"tETHUST"}"#,
            commands[1]
        );
    }

    #[test]
    fn multiple_msg_types_single_symbol() {
        let commands = get_ws_commands(
            &vec![MessageType::Trade, MessageType::L2Event],
            &vec!["tBTCUST".to_string()],
            true,
            None,
        );
        assert_eq!(commands.len(), 2);
        assert_eq!(
            r#"{"event":"subscribe", "channel":"trades", "symbol":"tBTCUST"}"#,
            commands[0]
        );
        assert_eq!(
            r#"{"event":"subscribe", "channel":"book", "symbol":"tBTCUST", "prec":"P0", "frec":"F0", "len":25}"#,
            commands[1]
        );
    }

    #[test]
    fn candlestick() {
        let mut configs = HashMap::new();
        configs.insert("interval".to_string(), "1m".to_string());
        let commands = get_ws_commands(
            &vec![MessageType::Candlestick],
            &vec!["tBTCUST".to_string(), "tETHUST".to_string()],
            true,
            Some(&configs),
        );
        assert_eq!(commands.len(), 2);
        assert_eq!(
            r#"{"event":"subscribe", "channel":"candles", "key":"trade:1m:tBTCUST"}"#,
            commands[0]
        );
        assert_eq!(
            r#"{"event":"subscribe", "channel":"candles", "key":"trade:1m:tETHUST"}"#,
            commands[1]
        );
    }
}
