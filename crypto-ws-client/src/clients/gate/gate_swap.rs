use async_trait::async_trait;

use super::utils::{GateCommandTranslator, GateMessageHandler, EXCHANGE_NAME};
use crate::{
    clients::common_traits::{
        Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO,
    },
    common::{command_translator::CommandTranslator, ws_client_internal::WSClientInternal},
    WSClient,
};

const INVERSE_SWAP_WEBSOCKET_URL: &str = "wss://fx-ws.gateio.ws/v4/ws/btc";
const LINEAR_SWAP_WEBSOCKET_URL: &str = "wss://fx-ws.gateio.ws/v4/ws/usdt";

/// The WebSocket client for Gate InverseSwap market.
///
/// * WebSocket API doc: <https://www.gate.io/docs/futures/ws/en/index.html>
/// * Trading at <https://www.gate.io/cn/futures_trade/BTC/BTC_USD>
pub struct GateInverseSwapWSClient {
    client: WSClientInternal<GateMessageHandler<'F'>>,
    translator: GateCommandTranslator<'F'>,
}

/// The WebSocket client for Gate LinearSwap market.
///
/// * WebSocket API doc: <https://www.gate.io/docs/futures/ws/en/index.html>
/// * Trading at <https://www.gate.io/cn/futures_trade/USDT/BTC_USDT>
pub struct GateLinearSwapWSClient {
    client: WSClientInternal<GateMessageHandler<'F'>>,
    translator: GateCommandTranslator<'F'>,
}

impl_new_constructor!(
    GateInverseSwapWSClient,
    EXCHANGE_NAME,
    INVERSE_SWAP_WEBSOCKET_URL,
    GateMessageHandler::<'F'> {},
    GateCommandTranslator::<'F'> {}
);

impl_new_constructor!(
    GateLinearSwapWSClient,
    EXCHANGE_NAME,
    LINEAR_SWAP_WEBSOCKET_URL,
    GateMessageHandler::<'F'> {},
    GateCommandTranslator::<'F'> {}
);

impl_trait!(Trade, GateInverseSwapWSClient, subscribe_trade, "trades");
#[rustfmt::skip]
impl_trait!(OrderBook, GateInverseSwapWSClient, subscribe_orderbook, "order_book_update");
#[rustfmt::skip]
impl_trait!(OrderBookTopK, GateInverseSwapWSClient, subscribe_orderbook_topk, "order_book");
impl_trait!(BBO, GateInverseSwapWSClient, subscribe_bbo, "book_ticker");
impl_trait!(Ticker, GateInverseSwapWSClient, subscribe_ticker, "tickers");

impl_trait!(Trade, GateLinearSwapWSClient, subscribe_trade, "trades");
#[rustfmt::skip]
impl_trait!(OrderBook, GateLinearSwapWSClient, subscribe_orderbook, "order_book_update");
#[rustfmt::skip]
impl_trait!(OrderBookTopK, GateLinearSwapWSClient, subscribe_orderbook_topk, "order_book");
impl_trait!(BBO, GateLinearSwapWSClient, subscribe_bbo, "book_ticker");
impl_trait!(Ticker, GateLinearSwapWSClient, subscribe_ticker, "tickers");

impl_candlestick!(GateInverseSwapWSClient);
impl_candlestick!(GateLinearSwapWSClient);

panic_l3_orderbook!(GateInverseSwapWSClient);
panic_l3_orderbook!(GateLinearSwapWSClient);

impl_ws_client_trait!(GateInverseSwapWSClient);
impl_ws_client_trait!(GateLinearSwapWSClient);
