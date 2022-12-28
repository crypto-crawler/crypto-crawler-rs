use std::collections::{BTreeMap, HashMap};

use crate::MessageType;

fn msg_type_to_channel(msg_type: MessageType) -> &'static str {
    match msg_type {
        MessageType::Trade => "trades",
        MessageType::L2Event => "books-l2-tbt",
        MessageType::L2TopK => "books5",
        MessageType::Ticker => "tickers",
        MessageType::Candlestick => "candle",
        _ => panic!("Unknown message type {}", msg_type),
    }
}

fn channel_symbol_to_topic(
    channel: &str,
    symbol: &str,
    configs: Option<&HashMap<String, String>>,
) -> String {
    if channel == "candle" {
        format!("candle{}:{}", configs.unwrap().get("interval").unwrap(), symbol)
    } else {
        format!("{}:{}", channel, symbol)
    }
}

fn topics_to_command(topics: &[String], subscribe: bool) -> String {
    let arr = topics
        .iter()
        .map(|s| {
            let mut map = BTreeMap::new();
            let v: Vec<&str> = s.split(':').collect();
            let channel = v[0];
            let symbol = v[1];
            map.insert("channel".to_string(), channel.to_string());
            map.insert("instId".to_string(), symbol.to_string());
            map
        })
        .collect::<Vec<BTreeMap<String, String>>>();
    format!(
        r#"{{"op":"{}","args":{}}}"#,
        if subscribe { "subscribe" } else { "unsubscribe" },
        serde_json::to_string(&arr).unwrap(),
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
            symbols.iter().map(|symbol| channel_symbol_to_topic(channel, symbol, configs))
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
            &["BTC-USDT-SWAP".to_string(), "ETH-USDT-SWAP".to_string()],
            true,
            None,
        );
        assert_eq!(commands.len(), 1);
        assert_eq!(
            r#"{"op":"subscribe","args":[{"channel":"trades","instId":"BTC-USDT-SWAP"},{"channel":"trades","instId":"ETH-USDT-SWAP"}]}"#,
            commands[0]
        );
    }

    #[test]
    fn multiple_msg_types_single_symbol() {
        let commands = get_ws_commands(
            &[MessageType::Trade, MessageType::L2Event],
            &["BTC-USDT-SWAP".to_string()],
            true,
            None,
        );
        assert_eq!(commands.len(), 1);
        assert_eq!(
            r#"{"op":"subscribe","args":[{"channel":"trades","instId":"BTC-USDT-SWAP"},{"channel":"books-l2-tbt","instId":"BTC-USDT-SWAP"}]}"#,
            commands[0]
        );
    }

    #[test]
    fn candlestick() {
        let mut configs = HashMap::new();
        configs.insert("interval".to_string(), "1m".to_string());
        let commands = get_ws_commands(
            &[MessageType::Candlestick],
            &["BTC-USDT-SWAP".to_string(), "ETH-USDT-SWAP".to_string()],
            true,
            Some(&configs),
        );
        assert_eq!(commands.len(), 1);
        assert_eq!(
            r#"{"op":"subscribe","args":[{"channel":"candle1m","instId":"BTC-USDT-SWAP"},{"channel":"candle1m","instId":"ETH-USDT-SWAP"}]}"#,
            commands[0]
        );
    }
}
