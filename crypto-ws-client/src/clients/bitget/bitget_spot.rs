use async_trait::async_trait;

use crate::{
    clients::common_traits::{
        Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO,
    },
    common::{command_translator::CommandTranslator, ws_client_internal::WSClientInternal},
    WSClient,
};

use super::{
    utils::{BitgetCommandTranslator, BitgetMessageHandler, UPLINK_LIMIT},
    EXCHANGE_NAME,
};

const WEBSOCKET_URL: &str = "wss://ws.bitget.com/spot/v1/stream";

/// The WebSocket client for Bitget Spot market.
///
/// * WebSocket API doc: <https://bitgetlimited.github.io/apidoc/en/spot/#websocketapi>
/// * Trading at: <https://www.bitget.com/en/spot/>
pub struct BitgetSpotWSClient {
    client: WSClientInternal<BitgetMessageHandler>,
    translator: BitgetCommandTranslator<'S'>,
}

impl BitgetSpotWSClient {
    pub async fn new(tx: std::sync::mpsc::Sender<String>, url: Option<&str>) -> Self {
        let real_url = match url {
            Some(endpoint) => endpoint,
            None => WEBSOCKET_URL,
        };
        BitgetSpotWSClient {
            client: WSClientInternal::connect(
                EXCHANGE_NAME,
                real_url,
                BitgetMessageHandler {},
                Some(UPLINK_LIMIT),
                tx,
            )
            .await,
            translator: BitgetCommandTranslator::<'S'> {},
        }
    }
}

impl_trait!(Trade, BitgetSpotWSClient, subscribe_trade, "trade");
#[rustfmt::skip]
impl_trait!(OrderBookTopK, BitgetSpotWSClient, subscribe_orderbook_topk, "books15");
impl_trait!(OrderBook, BitgetSpotWSClient, subscribe_orderbook, "books");
impl_trait!(Ticker, BitgetSpotWSClient, subscribe_ticker, "ticker");
impl_candlestick!(BitgetSpotWSClient);

panic_bbo!(BitgetSpotWSClient);
panic_l3_orderbook!(BitgetSpotWSClient);

impl_ws_client_trait!(BitgetSpotWSClient);
