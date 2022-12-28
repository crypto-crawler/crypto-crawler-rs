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

use super::{utils::fetch_symbol_id_map_spot, EXCHANGE_NAME};

const WEBSOCKET_URL: &str = "wss://kline.zbg.com/websocket";

/// The WebSocket client for ZBG spot market.
///
/// * WebSocket API doc: <https://www.zbg.com/docs/spot/v1/en/#websocket-market-data>
/// * Trading at: <https://www.zbg.com/trade/>
pub struct ZbgSpotWSClient {
    client: WSClientInternal<ZbgMessageHandler>,
    translator: ZbgCommandTranslator,
}

impl_new_constructor!(
    ZbgSpotWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    ZbgMessageHandler {},
    ZbgCommandTranslator::new().await
);

#[rustfmt::skip]
impl_trait!(Trade, ZbgSpotWSClient, subscribe_trade, "TRADE");
#[rustfmt::skip]
impl_trait!(OrderBook, ZbgSpotWSClient, subscribe_orderbook, "ENTRUST_ADD");
#[rustfmt::skip]
impl_trait!(Ticker, ZbgSpotWSClient, subscribe_ticker, "TRADE_STATISTIC_24H");
impl_candlestick!(ZbgSpotWSClient);

panic_bbo!(ZbgSpotWSClient);
panic_l2_topk!(ZbgSpotWSClient);
panic_l3_orderbook!(ZbgSpotWSClient);

impl_ws_client_trait!(ZbgSpotWSClient);

struct ZbgMessageHandler {}
struct ZbgCommandTranslator {
    symbol_id_map: HashMap<String, i64>,
}

impl MessageHandler for ZbgMessageHandler {
    fn handle_message(&mut self, msg: &str) -> MiscMessage {
        if msg.contains(r#"action":"PING"#) { MiscMessage::Pong } else { MiscMessage::Normal }
    }

    fn get_ping_msg_and_interval(&self) -> Option<(Message, u64)> {
        Some((Message::Text(r#"{"action":"PING"}"#.to_string()), 10))
    }
}

impl ZbgCommandTranslator {
    async fn new() -> Self {
        let symbol_id_map = fetch_symbol_id_map_spot().await;
        ZbgCommandTranslator { symbol_id_map }
    }

    fn to_raw_channel(&self, channel: &str, symbol: &str) -> String {
        let symbol_id = self
            .symbol_id_map
            .get(symbol.to_lowercase().as_str())
            .unwrap_or_else(|| panic!("Failed to find symbol_id for {}", symbol));
        if channel == "TRADE_STATISTIC_24H" {
            format!("{}_{}", symbol_id, channel)
        } else {
            format!("{}_{}_{}", symbol_id, channel, symbol.to_uppercase())
        }
    }

    fn to_candlestick_raw_channel(&self, symbol: &str, interval: usize) -> String {
        let interval_str = match interval {
            60 => "1M",
            300 => "5M",
            900 => "15M",
            1800 => "30M",
            3600 => "1H",
            14400 => "4H",
            86400 => "1D",
            604800 => "1W",
            _ => panic!("ZBG spot available intervals 1M,5M,15M,30M,1H,4H,1D,1W"),
        };

        let symbol_id = self
            .symbol_id_map
            .get(symbol.to_lowercase().as_str())
            .unwrap_or_else(|| panic!("Failed to find symbol_id for {}", symbol));

        format!("{}_KLINE_{}_{}", symbol_id, interval_str, symbol.to_uppercase())
    }
}

impl CommandTranslator for ZbgCommandTranslator {
    fn translate_to_commands(&self, subscribe: bool, topics: &[(String, String)]) -> Vec<String> {
        topics
            .iter()
            .map(|(channel, symbol)| {
                format!(
                    r#"{{"action":"{}", "dataType":{}}}"#,
                    if subscribe { "ADD" } else { "DEL" },
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
                    r#"{{"action":"{}", "dataType":{}}}"#,
                    if subscribe { "ADD" } else { "DEL" },
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
            .translate_to_commands(true, &[("TRADE".to_string(), "btc_usdt".to_string())]);

        assert_eq!(1, commands.len());
        assert_eq!(r#"{"action":"ADD", "dataType":329_TRADE_BTC_USDT}"#, commands[0]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_two_topic() {
        let translator = super::ZbgCommandTranslator::new().await;
        let commands = translator.translate_to_commands(
            true,
            &[
                ("TRADE".to_string(), "btc_usdt".to_string()),
                ("ENTRUST_ADD".to_string(), "eth_usdt".to_string()),
            ],
        );

        assert_eq!(2, commands.len());
        assert_eq!(r#"{"action":"ADD", "dataType":329_TRADE_BTC_USDT}"#, commands[0]);
        assert_eq!(r#"{"action":"ADD", "dataType":330_ENTRUST_ADD_ETH_USDT}"#, commands[1]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_candlestick() {
        let translator = super::ZbgCommandTranslator::new().await;
        let commands =
            translator.translate_to_candlestick_commands(true, &[("btc_usdt".to_string(), 60)]);

        assert_eq!(1, commands.len());
        assert_eq!(r#"{"action":"ADD", "dataType":329_KLINE_1M_BTC_USDT}"#, commands[0]);
    }
}
