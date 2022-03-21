use super::utils::{fetch_ws_token, KucoinMessageHandler, EXCHANGE_NAME, UPLINK_LIMIT};
use crate::{
    clients::common_traits::{
        Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO,
    },
    common::{command_translator::CommandTranslator, ws_client_internal::WSClientInternal},
    WSClient,
};
use async_trait::async_trait;
use std::sync::mpsc::Sender;

/// The WebSocket client for KuCoin Spot market.
///
/// * WebSocket API doc: <https://docs.kucoin.com/#websocket-feed>
/// * Trading at: <https://trade.kucoin.com/>
pub struct KuCoinSpotWSClient {
    client: WSClientInternal<KucoinMessageHandler>,
    translator: KucoinCommandTranslator,
}

impl KuCoinSpotWSClient {
    /// Creates a KuCoinSpotWSClient websocket client.
    ///
    /// # Arguments
    ///
    /// * `tx` - The sending part of a channel
    /// * `url` - Optional server url, usually you don't need specify it
    pub async fn new(tx: Sender<String>, url: Option<&str>) -> Self {
        let real_url = match url {
            Some(endpoint) => endpoint.to_string(),
            None => {
                let ws_token = fetch_ws_token().await;
                let ws_url = format!("{}?token={}", ws_token.endpoint, ws_token.token);
                ws_url
            }
        };
        KuCoinSpotWSClient {
            client: WSClientInternal::connect(
                EXCHANGE_NAME,
                &real_url,
                KucoinMessageHandler {},
                Some(UPLINK_LIMIT),
                tx,
            )
            .await,
            translator: KucoinCommandTranslator {},
        }
    }
}

impl_trait!(Trade, KuCoinSpotWSClient, subscribe_trade, "/market/match");
impl_trait!(BBO, KuCoinSpotWSClient, subscribe_bbo, "/market/ticker");
#[rustfmt::skip]
impl_trait!(OrderBook, KuCoinSpotWSClient, subscribe_orderbook, "/market/level2");
#[rustfmt::skip]
impl_trait!(OrderBookTopK, KuCoinSpotWSClient, subscribe_orderbook_topk, "/spotMarket/level2Depth5");
#[rustfmt::skip]
impl_trait!(Ticker, KuCoinSpotWSClient, subscribe_ticker, "/market/snapshot");
#[rustfmt::skip]
impl_trait!(Level3OrderBook, KuCoinSpotWSClient, subscribe_l3_orderbook, "/spotMarket/level3");

impl_candlestick!(KuCoinSpotWSClient);

impl_ws_client_trait!(KuCoinSpotWSClient);

struct KucoinCommandTranslator {}

impl KucoinCommandTranslator {
    fn to_candlestick_channel(symbol: &str, interval: usize) -> String {
        let interval_str = match interval {
            60 => "1min",
            180 => "3min",
            300 => "5min",
            900 => "15min",
            1800 => "30min",
            3600 => "1hour",
            7200 => "2hour",
            14400 => "4hour",
            21600 => "6hour",
            28800 => "8hour",
            43200 => "12hour",
            86400 => "1day",
            604800 => "1week",
            _ => panic!(
                "KuCoin available intervals 1min,3min,5min,15min,30min,1hour,2hour,4hour,6hour,8hour,12hour,1day,1week"
            ),
        };
        format!("{}_{}", symbol, interval_str)
    }
}

impl CommandTranslator for KucoinCommandTranslator {
    fn translate_to_commands(&self, subscribe: bool, topics: &[(String, String)]) -> Vec<String> {
        super::utils::topics_to_commands(topics, subscribe)
    }

    fn translate_to_candlestick_commands(
        &self,
        subscribe: bool,
        symbol_interval_list: &[(String, usize)],
    ) -> Vec<String> {
        let topics = symbol_interval_list
            .iter()
            .map(|(symbol, interval)| {
                (
                    "/market/candles".to_string(),
                    Self::to_candlestick_channel(symbol, *interval),
                )
            })
            .collect::<Vec<(String, String)>>();
        self.translate_to_commands(subscribe, &topics)
    }
}

#[cfg(test)]
mod tests {
    use crate::common::command_translator::CommandTranslator;

    #[test]
    fn test_one_channel() {
        let translator = super::KucoinCommandTranslator {};
        let commands = translator.translate_to_commands(
            true,
            &vec![("/market/match".to_string(), "BTC-USDT".to_string())],
        );

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"id":"crypto-ws-client","type":"subscribe","topic":"/market/match:BTC-USDT","privateChannel":false,"response":true}"#,
            commands[0]
        );

        let commands = translator.translate_to_commands(
            true,
            &vec![
                ("/market/match".to_string(), "BTC-USDT".to_string()),
                ("/market/match".to_string(), "ETH-USDT".to_string()),
            ],
        );

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"id":"crypto-ws-client","type":"subscribe","topic":"/market/match:BTC-USDT,ETH-USDT","privateChannel":false,"response":true}"#,
            commands[0]
        );
    }

    #[test]
    fn test_two_channels() {
        let translator = super::KucoinCommandTranslator {};
        let commands = translator.translate_to_commands(
            true,
            &vec![
                ("/market/match".to_string(), "BTC-USDT".to_string()),
                ("/market/level2".to_string(), "ETH-USDT".to_string()),
            ],
        );

        assert_eq!(2, commands.len());
        assert_eq!(
            r#"{"id":"crypto-ws-client","type":"subscribe","topic":"/market/level2:ETH-USDT","privateChannel":false,"response":true}"#,
            commands[0]
        );
        assert_eq!(
            r#"{"id":"crypto-ws-client","type":"subscribe","topic":"/market/match:BTC-USDT","privateChannel":false,"response":true}"#,
            commands[1]
        );

        let commands = translator.translate_to_commands(
            true,
            &vec![
                ("/market/match".to_string(), "BTC-USDT".to_string()),
                ("/market/match".to_string(), "ETH-USDT".to_string()),
                ("/market/level2".to_string(), "BTC-USDT".to_string()),
                ("/market/level2".to_string(), "ETH-USDT".to_string()),
            ],
        );

        assert_eq!(2, commands.len());
        assert_eq!(
            r#"{"id":"crypto-ws-client","type":"subscribe","topic":"/market/level2:BTC-USDT,ETH-USDT","privateChannel":false,"response":true}"#,
            commands[0]
        );
        assert_eq!(
            r#"{"id":"crypto-ws-client","type":"subscribe","topic":"/market/match:BTC-USDT,ETH-USDT","privateChannel":false,"response":true}"#,
            commands[1]
        );
    }

    #[test]
    fn test_candlestick() {
        let translator = super::KucoinCommandTranslator {};
        let commands = translator.translate_to_candlestick_commands(
            true,
            &vec![("BTC-USDT".to_string(), 180), ("ETH-USDT".to_string(), 60)],
        );

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"id":"crypto-ws-client","type":"subscribe","topic":"/market/candles:BTC-USDT_3min,ETH-USDT_1min","privateChannel":false,"response":true}"#,
            commands[0]
        );
    }
}
