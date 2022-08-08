use async_trait::async_trait;

use super::utils::{GateCommandTranslator, GateMessageHandler, EXCHANGE_NAME};
use crate::{
    clients::common_traits::{
        Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO,
    },
    common::{command_translator::CommandTranslator, ws_client_internal::WSClientInternal},
    WSClient,
};

const WEBSOCKET_URL: &str = "wss://api.gateio.ws/ws/v4/";

/// The WebSocket client for Gate spot market.
///
/// * WebSocket API doc: <https://www.gate.io/docs/developers/apiv4/ws/en/>
/// * Trading at <https://www.gate.io/trade/BTC_USDT>
pub struct GateSpotWSClient {
    client: WSClientInternal<GateMessageHandler<'S'>>,
    translator: GateCommandTranslator<'S'>,
}

impl_new_constructor!(
    GateSpotWSClient,
    EXCHANGE_NAME,
    WEBSOCKET_URL,
    GateMessageHandler::<'S'> {},
    GateCommandTranslator::<'S'> {}
);

impl_trait!(Trade, GateSpotWSClient, subscribe_trade, "trades");
#[rustfmt::skip]
impl_trait!(OrderBook, GateSpotWSClient, subscribe_orderbook, "order_book_update");
#[rustfmt::skip]
impl_trait!(OrderBookTopK, GateSpotWSClient, subscribe_orderbook_topk, "order_book");
impl_trait!(BBO, GateSpotWSClient, subscribe_bbo, "book_ticker");
impl_trait!(Ticker, GateSpotWSClient, subscribe_ticker, "tickers");

impl_candlestick!(GateSpotWSClient);

panic_l3_orderbook!(GateSpotWSClient);

impl_ws_client_trait!(GateSpotWSClient);
