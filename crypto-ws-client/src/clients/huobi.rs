use crate::WSClient;
use std::collections::HashMap;

use log::*;
use serde_json::Value;
use tungstenite::Message;

use super::ws_client_internal::{MiscMessage, WSClientInternal};

pub(super) const EXCHANGE_NAME: &str = "Huobi";

const SPOT_WEBSOCKET_URL: &str = "wss://api.huobi.pro/ws";
// const FUTURES_WEBSOCKET_URL: &str = "wss://www.hbdm.com/ws";
// const COIN_SWAP_WEBSOCKET_URL: &str = "wss://api.hbdm.com/swap-ws";
// const USDT_SWAP_WEBSOCKET_URL: &str = "wss://api.hbdm.com/linear-swap-ws";
// const OPTION_WEBSOCKET_URL: &str = "wss://api.hbdm.com/option-ws";
const FUTURES_WEBSOCKET_URL: &str = "wss://futures.huobi.com/ws";
const COIN_SWAP_WEBSOCKET_URL: &str = "wss://futures.huobi.com/swap-ws";
const USDT_SWAP_WEBSOCKET_URL: &str = "wss://futures.huobi.com/linear-swap-ws";
const OPTION_WEBSOCKET_URL: &str = "wss://futures.huobi.com/option-ws";

/// The WebSocket client for Huobi Spot market(<https://huobiapi.github.io/docs/spot/v1/en/>).
pub struct HuobiSpotWSClient<'a> {
    client: WSClientInternal<'a>,
}

/// The WebSocket client for Huobi Futures market(<https://huobiapi.github.io/docs/dm/v1/en/>).
pub struct HuobiFuturesWSClient<'a> {
    client: WSClientInternal<'a>,
}

/// The WebSocket client for Huobi Coin Swap market(<https://huobiapi.github.io/docs/coin_margined_swap/v1/en/>).
pub struct HuobiCoinSwapWSClient<'a> {
    client: WSClientInternal<'a>,
}

/// The WebSocket client for Huobi USDT Swap market(<https://huobiapi.github.io/docs/usdt_swap/v1/en/>).
pub struct HuobiUsdtSwapWSClient<'a> {
    client: WSClientInternal<'a>,
}
/// The WebSocket client for Huobi Option market(<https://huobiapi.github.io/docs/option/v1/en/>).
pub struct HuobiOptionWSClient<'a> {
    client: WSClientInternal<'a>,
}

fn channel_to_command(channel: &str, subscribe: bool) -> String {
    format!(
        r#"{{"{}":"{}","id":"crypto-ws-client"}}"#,
        if subscribe { "sub" } else { "unsub" },
        channel
    )
}

fn channels_to_commands(channels: &[String], subscribe: bool) -> Vec<String> {
    channels
        .iter()
        .map(|ch| channel_to_command(ch, subscribe))
        .collect()
}

fn on_misc_msg(msg: &str) -> MiscMessage {
    let resp = serde_json::from_str::<HashMap<String, Value>>(&msg);
    if resp.is_err() {
        error!("{} is not a JSON string, {}", msg, EXCHANGE_NAME);
        return MiscMessage::Misc;
    }
    let obj = resp.unwrap();

    if obj.contains_key("ping") {
        let value = obj.get("ping").unwrap();
        let mut pong_msg = HashMap::<String, &Value>::new();
        pong_msg.insert("pong".to_string(), value);
        let ws_msg = Message::Text(serde_json::to_string(&pong_msg).unwrap());
        return MiscMessage::WebSocket(ws_msg);
    }

    if let Some(status) = obj.get("status") {
        if status.as_str().unwrap() != "ok" {
            error!("Received {} from {}", msg, EXCHANGE_NAME);
            return MiscMessage::Misc;
        }
    }

    if !obj.contains_key("ch") || !obj.contains_key("ts") {
        warn!("Received {} from {}", msg, EXCHANGE_NAME);
        return MiscMessage::Misc;
    }
    MiscMessage::Normal
}

define_client!(
    HuobiSpotWSClient,
    EXCHANGE_NAME,
    SPOT_WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg
);
define_client!(
    HuobiFuturesWSClient,
    EXCHANGE_NAME,
    FUTURES_WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg
);
define_client!(
    HuobiCoinSwapWSClient,
    EXCHANGE_NAME,
    COIN_SWAP_WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg
);
define_client!(
    HuobiUsdtSwapWSClient,
    EXCHANGE_NAME,
    USDT_SWAP_WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg
);
define_client!(
    HuobiOptionWSClient,
    EXCHANGE_NAME,
    OPTION_WEBSOCKET_URL,
    channels_to_commands,
    on_misc_msg
);

#[cfg(test)]
mod tests {
    #[test]
    fn test_channel_to_command() {
        assert_eq!(
            r#"{"sub":"market.btcusdt.trade.detail","id":"crypto-ws-client"}"#,
            super::channel_to_command("market.btcusdt.trade.detail", true)
        );

        assert_eq!(
            r#"{"unsub":"market.btcusdt.trade.detail","id":"crypto-ws-client"}"#,
            super::channel_to_command("market.btcusdt.trade.detail", false)
        );
    }
}
