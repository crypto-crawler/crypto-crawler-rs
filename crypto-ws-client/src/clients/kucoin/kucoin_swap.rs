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

/// The WebSocket client for KuCoin Swap markets.
///
/// * WebSocket API doc: <https://docs.kucoin.cc/futures/#websocket-2>
/// * Trading at: <https://futures.kucoin.com/>
pub struct KuCoinSwapWSClient {
    client: WSClientInternal<KucoinMessageHandler>,
    translator: KucoinCommandTranslator,
}

impl KuCoinSwapWSClient {
    /// Creates a KuCoinSwapWSClient websocket client.
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
        KuCoinSwapWSClient {
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

#[rustfmt::skip]
impl_trait!(Trade, KuCoinSwapWSClient, subscribe_trade, "/contractMarket/execution");
#[rustfmt::skip]
impl_trait!(BBO, KuCoinSwapWSClient, subscribe_bbo, "/contractMarket/tickerV2");
#[rustfmt::skip]
impl_trait!(OrderBook, KuCoinSwapWSClient, subscribe_orderbook, "/contractMarket/level2");
#[rustfmt::skip]
impl_trait!(OrderBookTopK, KuCoinSwapWSClient, subscribe_orderbook_topk, "/contractMarket/level2Depth5");
#[rustfmt::skip]
impl_trait!(Ticker, KuCoinSwapWSClient, subscribe_ticker, "/contractMarket/snapshot");

impl_candlestick!(KuCoinSwapWSClient);

panic_l3_orderbook!(KuCoinSwapWSClient);

impl_ws_client_trait!(KuCoinSwapWSClient);

struct KucoinCommandTranslator {}

impl KucoinCommandTranslator {
    fn to_candlestick_channel(symbol: &str, interval: usize) -> String {
        let valid_set: Vec<usize> = vec![
            60, 300, 900, 1800, 3600, 7200, 14400, 28800, 43200, 86400, 604800,
        ];
        if !valid_set.contains(&interval) {
            let joined = valid_set
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",");
            panic!("KuCoin Swap available intervals {}", joined);
        }
        format!("{}_{}", symbol, interval / 60)
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
                    "/contractMarket/candle".to_string(),
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
            &[(
                "/contractMarket/execution".to_string(),
                "BTC_USD".to_string(),
            )],
        );

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"id":"crypto-ws-client","type":"subscribe","topic":"/contractMarket/execution:BTC_USD","privateChannel":false,"response":true}"#,
            commands[0]
        );

        let commands = translator.translate_to_commands(
            true,
            &[
                (
                    "/contractMarket/execution".to_string(),
                    "BTC_USD".to_string(),
                ),
                (
                    "/contractMarket/execution".to_string(),
                    "ETH_USD".to_string(),
                ),
            ],
        );

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"id":"crypto-ws-client","type":"subscribe","topic":"/contractMarket/execution:BTC_USD,ETH_USD","privateChannel":false,"response":true}"#,
            commands[0]
        );
    }

    #[test]
    fn test_two_channels() {
        let translator = super::KucoinCommandTranslator {};
        let commands = translator.translate_to_commands(
            true,
            &[
                (
                    "/contractMarket/execution".to_string(),
                    "BTC_USD".to_string(),
                ),
                ("/contractMarket/level2".to_string(), "ETH_USD".to_string()),
            ],
        );

        assert_eq!(2, commands.len());
        assert_eq!(
            r#"{"id":"crypto-ws-client","type":"subscribe","topic":"/contractMarket/execution:BTC_USD","privateChannel":false,"response":true}"#,
            commands[0]
        );
        assert_eq!(
            r#"{"id":"crypto-ws-client","type":"subscribe","topic":"/contractMarket/level2:ETH_USD","privateChannel":false,"response":true}"#,
            commands[1]
        );

        let commands = translator.translate_to_commands(
            true,
            &[
                (
                    "/contractMarket/execution".to_string(),
                    "BTC_USD".to_string(),
                ),
                (
                    "/contractMarket/execution".to_string(),
                    "ETH_USD".to_string(),
                ),
                ("/contractMarket/level2".to_string(), "BTC_USD".to_string()),
                ("/contractMarket/level2".to_string(), "ETH_USD".to_string()),
            ],
        );

        assert_eq!(2, commands.len());
        assert_eq!(
            r#"{"id":"crypto-ws-client","type":"subscribe","topic":"/contractMarket/execution:BTC_USD,ETH_USD","privateChannel":false,"response":true}"#,
            commands[0]
        );
        assert_eq!(
            r#"{"id":"crypto-ws-client","type":"subscribe","topic":"/contractMarket/level2:BTC_USD,ETH_USD","privateChannel":false,"response":true}"#,
            commands[1]
        );
    }

    #[test]
    fn test_candlestick() {
        let translator = super::KucoinCommandTranslator {};
        let commands = translator.translate_to_candlestick_commands(
            true,
            &[("BTC_USD".to_string(), 300), ("ETH_USD".to_string(), 60)],
        );

        assert_eq!(1, commands.len());
        assert_eq!(
            r#"{"id":"crypto-ws-client","type":"subscribe","topic":"/contractMarket/candle:BTC_USD_5,ETH_USD_1","privateChannel":false,"response":true}"#,
            commands[0]
        );
    }
}
