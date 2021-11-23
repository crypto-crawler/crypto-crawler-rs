use std::collections::HashMap;

use crate::MessageType;

fn msg_type_to_channel(msg_type: MessageType) -> &'static str {
    match msg_type {
        MessageType::Trade => "trade",
        MessageType::L2Event => "orderBookL2_25",
        MessageType::Ticker => "instrument_info.100ms",
        MessageType::Candlestick => "klineV2",
        _ => panic!("Unknown message type {}", msg_type),
    }
}

fn channel_symbol_to_topic(
    channel: &str,
    symbol: &str,
    configs: Option<&HashMap<String, String>>,
) -> String {
    if channel == "klineV2" {
        let interval_str = configs.unwrap().get("interval").unwrap();
        if symbol.ends_with("USDT") {
            format!("candle.{}.{}", interval_str, symbol)
        } else {
            format!("klineV2.{}.{}", interval_str, symbol)
        }
    } else {
        format!("{}.{}", channel, symbol)
    }
}

fn topics_to_command(topics: &[String], subscribe: bool) -> String {
    format!(
        r#"{{"op":"{}", "args":{}}}"#,
        if subscribe {
            "subscribe"
        } else {
            "unsubscribe"
        },
        serde_json::to_string(&topics).unwrap()
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
            &vec!["BTCUSD".to_string(), "ETHUSD".to_string()],
            true,
            None,
        );
        assert_eq!(commands.len(), 1);
        assert_eq!(
            r#"{"op":"subscribe", "args":["trade.BTCUSD","trade.ETHUSD"]}"#,
            commands[0]
        );
    }

    #[test]
    fn multiple_msg_types_single_symbol() {
        let commands = get_ws_commands(
            &vec![MessageType::Trade, MessageType::L2Event],
            &vec!["BTCUSD".to_string()],
            true,
            None,
        );
        assert_eq!(commands.len(), 1);
        assert_eq!(
            r#"{"op":"subscribe", "args":["trade.BTCUSD","orderBookL2_25.BTCUSD"]}"#,
            commands[0]
        );
    }

    #[test]
    fn candlestick() {
        let mut configs = HashMap::new();
        configs.insert("interval".to_string(), "1".to_string());
        let commands = get_ws_commands(
            &vec![MessageType::Candlestick],
            &vec!["BTCUSD".to_string(), "ETHUSD".to_string()],
            true,
            Some(&configs),
        );
        assert_eq!(commands.len(), 1);
        assert_eq!(
            r#"{"op":"subscribe", "args":["klineV2.1.BTCUSD","klineV2.1.ETHUSD"]}"#,
            commands[0]
        );
    }
}
