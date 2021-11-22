use std::collections::HashMap;

use crate::MessageType;

fn msg_type_to_channel(msg_type: MessageType) -> &'static str {
    match msg_type {
        MessageType::Trade => "aggTrade",
        MessageType::L2Event => "depth@100ms",
        MessageType::L2TopK => "depth5",
        MessageType::BBO => "bookTicker",
        MessageType::Ticker => "ticker",
        MessageType::Candlestick => "kline",
        _ => panic!("Unknown message type {}", msg_type),
    }
}

fn channel_symbol_to_topic(
    channel: &str,
    symbol: &str,
    configs: Option<&HashMap<String, String>>,
) -> String {
    if channel == "kline" {
        format!(
            "{}@kline_{}",
            symbol.to_lowercase(),
            configs.unwrap().get("interval").unwrap()
        )
    } else {
        format!("{}@{}", symbol.to_lowercase(), channel)
    }
}

fn topics_to_command(topics: &[String], subscribe: bool) -> String {
    format!(
        r#"{{"method":"{}","params":{}}}"#,
        if subscribe {
            "SUBSCRIBE"
        } else {
            "UNSUBSCRIBE"
        },
        serde_json::to_string(topics).unwrap()
    )
}

pub(crate) fn get_ws_commands(
    msg_types: &[MessageType],
    symbols: &[String],
    subscribe: bool,
    configs: Option<&HashMap<String, String>>,
) -> Vec<String> {
    let topics = msg_types
        .iter()
        .map(|msg_type| msg_type_to_channel(*msg_type))
        .flat_map(|channel| {
            symbols
                .iter()
                .map(|symbol| channel_symbol_to_topic(channel, symbol, configs))
        })
        .collect::<Vec<String>>();
    vec![topics_to_command(&topics, subscribe)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_msg_type_multiple_symbols() {
        let commands = get_ws_commands(
            &vec![MessageType::Trade],
            &vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()],
            true,
            None,
        );
        assert_eq!(commands.len(), 1);
        assert_eq!(
            r#"{"method":"SUBSCRIBE","params":["btcusdt@aggTrade","ethusdt@aggTrade"]}"#,
            commands[0]
        );
    }

    #[test]
    fn multiple_msg_types_single_symbol() {
        let commands = get_ws_commands(
            &vec![MessageType::Trade, MessageType::L2Event],
            &vec!["BTCUSDT".to_string()],
            true,
            None,
        );
        assert_eq!(commands.len(), 1);
        assert_eq!(
            r#"{"method":"SUBSCRIBE","params":["btcusdt@aggTrade","btcusdt@depth@100ms"]}"#,
            commands[0]
        );
    }

    #[test]
    fn candlestick() {
        let mut configs = HashMap::new();
        configs.insert("interval".to_string(), "1m".to_string());
        let commands = get_ws_commands(
            &vec![MessageType::Candlestick],
            &vec!["BTCUSDT".to_string()],
            true,
            Some(&configs),
        );
        assert_eq!(commands.len(), 1);
        assert_eq!(
            r#"{"method":"SUBSCRIBE","params":["btcusdt@kline_1m"]}"#,
            commands[0]
        );
    }
}
