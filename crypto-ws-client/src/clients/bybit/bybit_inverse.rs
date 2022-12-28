use async_trait::async_trait;

use crate::{
    clients::common_traits::{
        Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO,
    },
    common::{command_translator::CommandTranslator, ws_client_internal::WSClientInternal},
    WSClient,
};

use super::utils::{BybitMessageHandler, EXCHANGE_NAME};

const WEBSOCKET_URL: &str = "wss://stream.bybit.com/realtime";

/// Bybit Inverses markets.
///
/// InverseFuture:
///   * WebSocket API doc: <https://bybit-exchange.github.io/docs/inverse_futures/>
///   * Trading at: <https://www.bybit.com/trade/inverse/futures/BTCUSD_BIQ>
///
/// InverseSwap:
///   * WebSocket API doc: <https://bybit-exchange.github.io/docs/inverse/#t-websocket>
///   * Trading at: <https://www.bybit.com/trade/inverse/>
pub struct BybitInverseWSClient {
    client: WSClientInternal<BybitMessageHandler>,
    translator: BybitInverseCommandTranslator,
}

impl_new_constructor!(
    BybitInverseWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    BybitMessageHandler {},
    BybitInverseCommandTranslator {}
);

impl_trait!(Trade, BybitInverseWSClient, subscribe_trade, "trade");
#[rustfmt::skip]
// Prefer orderBookL2_25 over orderBook_200.100ms because /public/orderBook/L2
// returns a top 25 snapshot, which is the same depth as orderBookL2_25.
impl_trait!(OrderBook, BybitInverseWSClient, subscribe_orderbook, "orderBookL2_25");
#[rustfmt::skip]
impl_trait!(Ticker, BybitInverseWSClient, subscribe_ticker, "instrument_info.100ms");
impl_candlestick!(BybitInverseWSClient);
panic_bbo!(BybitInverseWSClient);
panic_l3_orderbook!(BybitInverseWSClient);
panic_l2_topk!(BybitInverseWSClient);

impl_ws_client_trait!(BybitInverseWSClient);

struct BybitInverseCommandTranslator {}

impl BybitInverseCommandTranslator {
    // https://bybit-exchange.github.io/docs/inverse_futures/#t-websocketklinev2
    // https://bybit-exchange.github.io/docs/inverse/#t-websocketklinev2
    fn to_candlestick_raw_channel(interval: usize) -> String {
        let interval_str = match interval {
            60 => "1",
            180 => "3",
            300 => "5",
            900 => "15",
            1800 => "30",
            3600 => "60",
            7200 => "120",
            14400 => "240",
            21600 => "360",
            86400 => "D",
            604800 => "W",
            2592000 => "M",
            _ => panic!(
                "Bybit InverseFuture has intervals 1min,5min,15min,30min,60min,4hour,1day,1week,1mon"
            ),
        };
        format!("klineV2.{interval_str}")
    }
}

impl CommandTranslator for BybitInverseCommandTranslator {
    fn translate_to_commands(&self, subscribe: bool, topics: &[(String, String)]) -> Vec<String> {
        vec![super::utils::topics_to_command(topics, subscribe)]
    }

    fn translate_to_candlestick_commands(
        &self,
        subscribe: bool,
        symbol_interval_list: &[(String, usize)],
    ) -> Vec<String> {
        let topics = symbol_interval_list
            .iter()
            .map(|(symbol, interval)| {
                let channel = Self::to_candlestick_raw_channel(*interval);
                (channel, symbol.to_string())
            })
            .collect::<Vec<(String, String)>>();
        self.translate_to_commands(subscribe, &topics)
    }
}
