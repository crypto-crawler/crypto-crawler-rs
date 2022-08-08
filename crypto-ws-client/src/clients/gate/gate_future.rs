use async_trait::async_trait;

use super::utils::{GateCommandTranslator, GateMessageHandler, EXCHANGE_NAME};
use crate::{
    clients::common_traits::{
        Candlestick, Level3OrderBook, OrderBook, OrderBookTopK, Ticker, Trade, BBO,
    },
    common::{command_translator::CommandTranslator, ws_client_internal::WSClientInternal},
    WSClient,
};

const INVERSE_FUTURE_WEBSOCKET_URL: &str = "wss://fx-ws.gateio.ws/v4/ws/delivery/btc";
const LINEAR_FUTURE_WEBSOCKET_URL: &str = "wss://fx-ws.gateio.ws/v4/ws/delivery/usdt";

/// The WebSocket client for Gate InverseFuture market.
///
/// * WebSocket API doc: <https://www.gate.io/docs/developers/delivery/ws/en/>
/// * Trading at <https://www.gate.io/futures-delivery/btc>
pub struct GateInverseFutureWSClient {
    client: WSClientInternal<GateMessageHandler<'F'>>,
    translator: GateCommandTranslator<'F'>,
}

/// The WebSocket client for Gate LinearFuture market.
///
/// * WebSocket API doc: <https://www.gate.io/docs/developers/delivery/ws/en/>
/// * Trading at <https://www.gate.io/futures-delivery/usdt>
pub struct GateLinearFutureWSClient {
    client: WSClientInternal<GateMessageHandler<'F'>>,
    translator: GateCommandTranslator<'F'>,
}

impl_new_constructor!(
    GateInverseFutureWSClient,
    EXCHANGE_NAME,
    INVERSE_FUTURE_WEBSOCKET_URL,
    GateMessageHandler::<'F'> {},
    GateCommandTranslator::<'F'> {}
);

impl_new_constructor!(
    GateLinearFutureWSClient,
    EXCHANGE_NAME,
    LINEAR_FUTURE_WEBSOCKET_URL,
    GateMessageHandler::<'F'> {},
    GateCommandTranslator::<'F'> {}
);

impl_trait!(Trade, GateInverseFutureWSClient, subscribe_trade, "trades");
#[rustfmt::skip]
impl_trait!(OrderBook, GateInverseFutureWSClient, subscribe_orderbook, "order_book");
#[rustfmt::skip]
impl_trait!(Ticker, GateInverseFutureWSClient, subscribe_ticker, "tickers");

#[rustfmt::skip]
impl_trait!(Trade, GateLinearFutureWSClient, subscribe_trade, "trades");
#[rustfmt::skip]
impl_trait!(OrderBook, GateLinearFutureWSClient, subscribe_orderbook, "order_book");
#[rustfmt::skip]
impl_trait!(Ticker, GateLinearFutureWSClient, subscribe_ticker, "tickers");

impl_candlestick!(GateInverseFutureWSClient);
impl_candlestick!(GateLinearFutureWSClient);

panic_bbo!(GateInverseFutureWSClient);
panic_bbo!(GateLinearFutureWSClient);
panic_l2_topk!(GateInverseFutureWSClient);
panic_l2_topk!(GateLinearFutureWSClient);
panic_l3_orderbook!(GateInverseFutureWSClient);
panic_l3_orderbook!(GateLinearFutureWSClient);

impl_ws_client_trait!(GateInverseFutureWSClient);
impl_ws_client_trait!(GateLinearFutureWSClient);
