use std::collections::HashMap;

use crate::MessageType;

fn msg_type_to_channel(msg_type: MessageType) -> &'static str {
    match msg_type {
        MessageType::Trade => "trades",
        MessageType::L2Event => "orderbook",
        MessageType::BBO => "ticker",
        _ => panic!("Unknown message type {msg_type}"),
    }
}

fn channel_symbol_to_command(channel: &str, symbol: &str, subscribe: bool) -> String {
    format!(
        r#"{{"op":"{}","channel":"{}","market":"{}"}}"#,
        if subscribe { "subscribe" } else { "unsubscribe" },
        channel,
        symbol,
    )
}

pub(crate) fn get_ws_commands(
    msg_types: &[MessageType],
    symbols: &[String],
    subscribe: bool,
    _configs: Option<&HashMap<String, String>>,
) -> Vec<String> {
    msg_types
        .iter()
        .map(|msg_type| msg_type_to_channel(*msg_type))
        .flat_map(|channel| {
            symbols.iter().map(|symbol| channel_symbol_to_command(channel, symbol, subscribe))
        })
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_msg_type_multiple_symbols() {
        let commands = get_ws_commands(
            &[MessageType::Trade],
            &["BTC/USD".to_string(), "BTC-PERP".to_string()],
            true,
            None,
        );
        assert_eq!(commands.len(), 2);
        println!("{}", commands[0]);
        assert_eq!(r#"{"op":"subscribe","channel":"trades","market":"BTC/USD"}"#, commands[0]);
        assert_eq!(r#"{"op":"subscribe","channel":"trades","market":"BTC-PERP"}"#, commands[1]);
    }

    #[test]
    fn multiple_msg_types_single_symbol() {
        let commands = get_ws_commands(
            &[MessageType::Trade, MessageType::L2Event],
            &["BTC-PERP".to_string()],
            true,
            None,
        );
        assert_eq!(commands.len(), 2);
        assert_eq!(r#"{"op":"subscribe","channel":"trades","market":"BTC-PERP"}"#, commands[0]);
        assert_eq!(r#"{"op":"subscribe","channel":"orderbook","market":"BTC-PERP"}"#, commands[1]);
    }
}
