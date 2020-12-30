use crate::WSClient;
use std::collections::HashMap;

use super::ws_client_internal::WSClientInternal;

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

fn serialize_command(channels: &[String], subscribe: bool) -> Vec<String> {
    channels
        .iter()
        .map(|ch| {
            let mut command = HashMap::new();
            command.insert(if subscribe { "sub" } else { "unsub" }, ch.as_str());
            command.insert("id", "crypto-ws-client");
            command
        })
        .map(|m| serde_json::to_string(&m).unwrap())
        .collect::<Vec<String>>()
}

define_client!(
    HuobiSpotWSClient,
    EXCHANGE_NAME,
    SPOT_WEBSOCKET_URL,
    serialize_command
);
define_client!(
    HuobiFuturesWSClient,
    EXCHANGE_NAME,
    FUTURES_WEBSOCKET_URL,
    serialize_command
);
define_client!(
    HuobiCoinSwapWSClient,
    EXCHANGE_NAME,
    COIN_SWAP_WEBSOCKET_URL,
    serialize_command
);
define_client!(
    HuobiUsdtSwapWSClient,
    EXCHANGE_NAME,
    USDT_SWAP_WEBSOCKET_URL,
    serialize_command
);
define_client!(
    HuobiOptionWSClient,
    EXCHANGE_NAME,
    OPTION_WEBSOCKET_URL,
    serialize_command
);
