use std::collections::HashMap;

use crate::MessageType;

fn msg_type_symbol_to_topic(
    msg_type: MessageType,
    symbol: &str,
    configs: Option<&HashMap<String, String>>,
) -> String {
    let is_spot = symbol.to_lowercase() == *symbol;
    let channel = match msg_type {
        MessageType::Trade => "trade.detail",
        MessageType::L2Event => {
            if is_spot {
                "mbp.20"
            } else {
                "depth.size_20.high_freq"
            }
        }
        MessageType::L2TopK => {
            if is_spot {
                "depth.step1"
            } else {
                "depth.step7"
            }
        }
        MessageType::BBO => "bbo",
        MessageType::Ticker => "detail",
        MessageType::Candlestick => "kline",
        _ => panic!("Unknown message type {}", msg_type),
    };
    if msg_type == MessageType::Candlestick {
        format!(
            "market.{}.kline.{}",
            symbol,
            configs.unwrap().get("interval").unwrap()
        )
    } else {
        format!("market.{}.{}", symbol, channel)
    }
}

fn topic_to_command(topic: &str, subscribe: bool) -> String {
    if topic.ends_with("depth.size_20.high_freq") {
        format!(
            r#"{{"{}": "{}","data_type":"incremental","id": "crypto-ws-client"}}"#,
            if subscribe { "sub" } else { "unsub" },
            topic,
        )
    } else {
        format!(
            r#"{{"{}":"{}","id":"crypto-ws-client"}}"#,
            if subscribe { "sub" } else { "unsub" },
            topic,
        )
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
                .map(|symbol| msg_type_symbol_to_topic(*msg_type, symbol, configs))
        })
        .map(|topic| topic_to_command(&topic, subscribe))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_msg_type_multiple_symbols() {
        let commands = get_ws_commands(
            &vec![MessageType::Trade],
            &vec!["BTC-USD".to_string(), "ETH-USD".to_string()],
            true,
            None,
        );
        assert_eq!(commands.len(), 2);
        assert_eq!(
            r#"{"sub":"market.BTC-USD.trade.detail","id":"crypto-ws-client"}"#,
            commands[0]
        );
        assert_eq!(
            r#"{"sub":"market.ETH-USD.trade.detail","id":"crypto-ws-client"}"#,
            commands[1]
        );
    }

    #[test]
    fn multiple_msg_types_single_symbol() {
        let commands = get_ws_commands(
            &vec![MessageType::Trade, MessageType::L2Event],
            &vec!["BTC-USD".to_string()],
            true,
            None,
        );
        assert_eq!(commands.len(), 2);
        assert_eq!(
            r#"{"sub":"market.BTC-USD.trade.detail","id":"crypto-ws-client"}"#,
            commands[0]
        );
        assert_eq!(
            r#"{"sub": "market.BTC-USD.depth.size_20.high_freq","data_type":"incremental","id": "crypto-ws-client"}"#,
            commands[1]
        );
    }

    #[test]
    fn candlestick() {
        let mut configs = HashMap::new();
        configs.insert("interval".to_string(), "1m".to_string());
        let commands = get_ws_commands(
            &vec![MessageType::Candlestick],
            &vec!["BTC-USD".to_string(), "ETH-USD".to_string()],
            true,
            Some(&configs),
        );
        assert_eq!(commands.len(), 2);
        assert_eq!(
            r#"{"sub":"market.BTC-USD.kline.1m","id":"crypto-ws-client"}"#,
            commands[0]
        );
        assert_eq!(
            r#"{"sub":"market.ETH-USD.kline.1m","id":"crypto-ws-client"}"#,
            commands[1]
        );
    }
}
