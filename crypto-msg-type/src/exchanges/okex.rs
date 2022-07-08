use std::collections::HashMap;

use crate::MessageType;

fn get_market_type(symbol: &str) -> &'static str {
    if symbol.ends_with("-SWAP") {
        "swap"
    } else {
        let c = symbol.matches('-').count();
        if c == 1 {
            "spot"
        } else if c == 2 {
            let date = &symbol[(symbol.len() - 6)..];
            debug_assert!(date.parse::<i64>().is_ok());
            "futures"
        } else {
            debug_assert!(symbol.ends_with("-C") || symbol.ends_with("-P"));
            "option"
        }
    }
}

fn msg_type_to_channel(msg_type: MessageType) -> &'static str {
    match msg_type {
        MessageType::Trade => "trade",
        MessageType::L2Event => "depth_l2_tbt",
        MessageType::L2TopK => "depth5",
        MessageType::BBO => "ticker",
        MessageType::Ticker => "ticker",
        MessageType::Candlestick => "candle",
        _ => panic!("Unknown message type {}", msg_type),
    }
}

fn channel_symbol_to_topic(
    channel: &str,
    symbol: &str,
    configs: Option<&HashMap<String, String>>,
) -> String {
    let market_type = get_market_type(symbol);
    if channel == "candle" {
        format!(
            "{}/candle{}s:{}",
            market_type,
            configs.unwrap().get("interval").unwrap(),
            symbol
        )
    } else {
        format!("{}/{}:{}", market_type, channel, symbol)
    }
}

fn topics_to_command(topics: &[String], subscribe: bool) -> String {
    format!(
        r#"{{"op":"{}","args":{}}}"#,
        if subscribe {
            "subscribe"
        } else {
            "unsubscribe"
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
            &[MessageType::Trade],
            &["BTC-USDT-SWAP".to_string(), "ETH-USDT-SWAP".to_string()],
            true,
            None,
        );
        assert_eq!(commands.len(), 1);
        assert_eq!(
            r#"{"op":"subscribe","args":["swap/trade:BTC-USDT-SWAP","swap/trade:ETH-USDT-SWAP"]}"#,
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
            r#"{"op":"subscribe","args":["swap/trade:BTC-USDT-SWAP","swap/depth_l2_tbt:BTC-USDT-SWAP"]}"#,
            commands[0]
        );
    }

    #[test]
    fn candlestick() {
        let mut configs = HashMap::new();
        configs.insert("interval".to_string(), "60".to_string());
        let commands = get_ws_commands(
            &[MessageType::Candlestick],
            &["BTC-USDT-SWAP".to_string(), "ETH-USDT-SWAP".to_string()],
            true,
            Some(&configs),
        );
        assert_eq!(commands.len(), 1);
        assert_eq!(
            r#"{"op":"subscribe","args":["swap/candle60s:BTC-USDT-SWAP","swap/candle60s:ETH-USDT-SWAP"]}"#,
            commands[0]
        );
    }
}
