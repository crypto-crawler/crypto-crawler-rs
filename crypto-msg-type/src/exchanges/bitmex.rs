use std::collections::HashMap;

use crate::MessageType;

fn msg_type_to_channel(msg_type: MessageType) -> &'static str {
    match msg_type {
        MessageType::Trade => "trade",
        MessageType::L2Event => "orderBookL2_25",
        MessageType::L2TopK => "orderBook10",
        MessageType::BBO => "quote",
        MessageType::Candlestick => "tradeBin",
        _ => panic!("Unknown message type {}", msg_type),
    }
}

fn channel_symbol_to_topic(
    channel: &str,
    symbol: &str,
    configs: Option<&HashMap<String, String>>,
) -> String {
    if channel == "tradeBin" {
        format!(
            "tradeBin{}:{}",
            configs.unwrap().get("interval").unwrap(),
            symbol
        )
    } else {
        format!("{}:{}", channel, symbol)
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
            &[MessageType::Trade],
            &["XBTUSD".to_string(), "ETHUSD".to_string()],
            true,
            None,
        );
        assert_eq!(commands.len(), 1);
        assert_eq!(
            r#"{"op":"subscribe", "args":["trade:XBTUSD","trade:ETHUSD"]}"#,
            commands[0]
        );
    }

    #[test]
    fn multiple_msg_types_single_symbol() {
        let commands = get_ws_commands(
            &[MessageType::Trade, MessageType::L2Event],
            &["XBTUSD".to_string()],
            true,
            None,
        );
        assert_eq!(commands.len(), 1);
        assert_eq!(
            r#"{"op":"subscribe", "args":["trade:XBTUSD","orderBookL2_25:XBTUSD"]}"#,
            commands[0]
        );
    }

    #[test]
    fn candlestick() {
        let mut configs = HashMap::new();
        configs.insert("interval".to_string(), "1m".to_string());
        let commands = get_ws_commands(
            &[MessageType::Candlestick],
            &["XBTUSD".to_string(), "ETHUSD".to_string()],
            true,
            Some(&configs),
        );
        assert_eq!(commands.len(), 1);
        assert_eq!(
            r#"{"op":"subscribe", "args":["tradeBin1m:XBTUSD","tradeBin1m:ETHUSD"]}"#,
            commands[0]
        );
    }
}
