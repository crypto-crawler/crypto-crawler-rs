use std::collections::HashMap;

use crate::MessageType;

fn msg_type_symbol_to_topic(
    msg_type: MessageType,
    symbol: &str,
    configs: Option<&HashMap<String, String>>,
) -> String {
    match msg_type {
        MessageType::Trade => format!("trades.{}.100ms", symbol),
        MessageType::L2Event => format!("book.{}.100ms", symbol),
        MessageType::L2TopK => format!("book.{}.5.10.100ms", symbol),
        MessageType::BBO => format!("quote.{}", symbol),
        MessageType::Candlestick => format!(
            "chart.trades.{}.{}",
            symbol,
            configs.unwrap().get("interval").unwrap()
        ),
        MessageType::Ticker => format!("ticker.{}.100ms", symbol),
        _ => panic!("Unknown message type {}", msg_type),
    }
}

fn topics_to_command(topics: &[String], subscribe: bool) -> String {
    format!(
        r#"{{"method":"public/{}", "params":{{"channels":{}}}}}"#,
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
        .flat_map(|msg_type| {
            symbols
                .iter()
                .map(|symbol| msg_type_symbol_to_topic(*msg_type, symbol, configs))
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
            &["BTC-PERPETUAL".to_string(), "ETH-PERPETUAL".to_string()],
            true,
            None,
        );
        assert_eq!(commands.len(), 1);
        assert_eq!(
            r#"{"method":"public/subscribe", "params":{"channels":["trades.BTC-PERPETUAL.100ms","trades.ETH-PERPETUAL.100ms"]}}"#,
            commands[0]
        );
    }

    #[test]
    fn multiple_msg_types_single_symbol() {
        let commands = get_ws_commands(
            &[MessageType::Trade, MessageType::L2Event],
            &["BTC-PERPETUAL".to_string()],
            true,
            None,
        );
        assert_eq!(commands.len(), 1);
        assert_eq!(
            r#"{"method":"public/subscribe", "params":{"channels":["trades.BTC-PERPETUAL.100ms","book.BTC-PERPETUAL.100ms"]}}"#,
            commands[0]
        );
    }

    #[test]
    fn candlestick() {
        let mut configs = HashMap::new();
        configs.insert("interval".to_string(), "1m".to_string());
        let commands = get_ws_commands(
            &[MessageType::Candlestick],
            &["BTC-PERPETUAL".to_string(), "ETH-PERPETUAL".to_string()],
            true,
            Some(&configs),
        );
        assert_eq!(commands.len(), 1);
        assert_eq!(
            r#"{"method":"public/subscribe", "params":{"channels":["chart.trades.BTC-PERPETUAL.1m","chart.trades.ETH-PERPETUAL.1m"]}}"#,
            commands[0]
        );
    }
}
