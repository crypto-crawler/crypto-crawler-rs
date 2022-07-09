use async_trait::async_trait;
use std::collections::HashMap;
use tokio_tungstenite::tungstenite::Message;

use crate::{
    clients::common_traits::{
        Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO,
    },
    common::{
        command_translator::CommandTranslator,
        message_handler::{MessageHandler, MiscMessage},
        ws_client_internal::WSClientInternal,
    },
    WSClient,
};

use log::*;

use super::{utils::fetch_symbol_contract_id_map_swap, EXCHANGE_NAME};

const WEBSOCKET_URL: &str = "wss://kline.zbg.com/exchange/v1/futurews";

/// The WebSocket client for ZBG swap market.
///
/// * WebSocket API doc: <https://www.zbgpro.com/docs/future/v1/cn/#300f34d976>, there is no English doc
/// * Trading at: <https://futures.zbg.com/>
pub struct ZbgSwapWSClient {
    client: WSClientInternal<ZbgMessageHandler>,
    translator: ZbgCommandTranslator,
}

impl_new_constructor!(
    ZbgSwapWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    ZbgMessageHandler {},
    ZbgCommandTranslator::new().await
);

#[rustfmt::skip]
impl_trait!(Trade, ZbgSwapWSClient, subscribe_trade, "future_tick");
#[rustfmt::skip]
impl_trait!(OrderBook, ZbgSwapWSClient, subscribe_orderbook, "future_snapshot_depth");
#[rustfmt::skip]
impl_trait!(Ticker, ZbgSwapWSClient, subscribe_ticker, "future_snapshot_indicator");
impl_candlestick!(ZbgSwapWSClient);

panic_bbo!(ZbgSwapWSClient);
panic_l2_topk!(ZbgSwapWSClient);
panic_l3_orderbook!(ZbgSwapWSClient);

impl_ws_client_trait!(ZbgSwapWSClient);

struct ZbgMessageHandler {}
struct ZbgCommandTranslator {
    symbol_id_map: HashMap<String, i64>,
}

impl MessageHandler for ZbgMessageHandler {
    fn handle_message(&mut self, msg: &str) -> MiscMessage {
        if msg == "Pong" {
            return MiscMessage::Pong;
        }
        if msg.starts_with('[') {
            MiscMessage::Normal
        } else {
            warn!("Received {} from {}", msg, EXCHANGE_NAME);
            MiscMessage::Other
        }
    }

    fn get_ping_msg_and_interval(&self) -> Option<(Message, u64)> {
        Some((Message::Text("PING".to_string()), 25))
    }
}

impl ZbgCommandTranslator {
    async fn new() -> Self {
        let symbol_id_map = fetch_symbol_contract_id_map_swap().await;
        ZbgCommandTranslator { symbol_id_map }
    }

    fn to_raw_channel(&self, channel: &str, symbol: &str) -> String {
        let contract_id = self
            .symbol_id_map
            .get(symbol)
            .unwrap_or_else(|| panic!("Failed to find contract_id for {}", symbol));
        format!("{}-{}", channel, contract_id)
    }

    fn to_candlestick_raw_channel(&self, pair: &str, interval: usize) -> String {
        let valid_set: Vec<usize> = vec![
            60, 180, 300, 900, 1800, 3600, 7200, 14400, 21600, 43200, 86400, 604800,
        ];
        if !valid_set.contains(&interval) {
            let joined = valid_set
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",");
            panic!("ZBG Swap available intervals {}", joined);
        }

        let contract_id = self
            .symbol_id_map
            .get(pair)
            .unwrap_or_else(|| panic!("Failed to find contract_id for {}", pair));

        format!("future_kline-{}-{}", contract_id, interval * 1000)
    }
}

impl CommandTranslator for ZbgCommandTranslator {
    fn translate_to_commands(&self, subscribe: bool, topics: &[(String, String)]) -> Vec<String> {
        topics
            .iter()
            .map(|(channel, symbol)| {
                format!(
                    r#"{{"action":"{}", "topic":"{}"}}"#,
                    if subscribe { "sub" } else { "unsub" },
                    self.to_raw_channel(channel, symbol),
                )
            })
            .collect()
    }

    fn translate_to_candlestick_commands(
        &self,
        subscribe: bool,
        symbol_interval_list: &[(String, usize)],
    ) -> Vec<String> {
        symbol_interval_list
            .iter()
            .map(|(symbol, interval)| {
                format!(
                    r#"{{"action":"{}", "topic":"{}"}}"#,
                    if subscribe { "sub" } else { "unsub" },
                    self.to_candlestick_raw_channel(symbol, *interval),
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::common::command_translator::CommandTranslator;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_one_topic() {
        let translator = super::ZbgCommandTranslator::new().await;
        let commands = translator
            .translate_to_commands(true, &[("future_tick".to_string(), "BTC_USDT".to_string())]);

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"action":"sub", "topic":"future_tick-1000000"}"#,
            commands[0]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_two_topic() {
        let translator = super::ZbgCommandTranslator::new().await;
        let commands = translator.translate_to_commands(
            true,
            &[
                ("future_tick".to_string(), "BTC_USDT".to_string()),
                ("future_snapshot_depth".to_string(), "ETH_USDT".to_string()),
            ],
        );

        assert_eq!(2, commands.len());
        assert_eq!(
            r#"{"action":"sub", "topic":"future_tick-1000000"}"#,
            commands[0]
        );
        assert_eq!(
            r#"{"action":"sub", "topic":"future_snapshot_depth-1000002"}"#,
            commands[1]
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_candlestick() {
        let translator = super::ZbgCommandTranslator::new().await;
        let commands =
            translator.translate_to_candlestick_commands(true, &[("BTC_USDT".to_string(), 60)]);

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"action":"sub", "topic":"future_kline-1000000-60000"}"#,
            commands[0]
        );
    }
}
