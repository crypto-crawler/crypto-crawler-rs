use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc, Mutex,
};

use std::time::Duration;

use super::utils::{check_args, fetch_symbols_retry};
use crate::{msg::Message, MessageType};
use crypto_markets::MarketType;
use crypto_rest_client::*;
use crypto_ws_client::*;
use log::*;

const EXCHANGE_NAME: &str = "okex";
// https://www.okex.com/docs/zh/#question-public
// How many subscriptions per websocket connection?
// The total size of subscription command should not exceed 4096 bytes.
const MAX_SUBSCRIPTIONS_PER_CONNECTION: usize = 256;

#[rustfmt::skip]
gen_crawl_event!(crawl_trade_internal, OkexWSClient, MessageType::Trade, subscribe_trade);

pub(crate) fn crawl_trade(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    if market_type == MarketType::EuropeanOption
        && (symbols.is_none() || symbols.unwrap().is_empty())
    {
        let on_msg_ext = Arc::new(Mutex::new(move |msg: String| {
            let message = Message::new(
                EXCHANGE_NAME.to_string(),
                market_type,
                MessageType::Trade,
                msg,
            );
            (on_msg.lock().unwrap())(message);
        }));

        let underlying = OkexRestClient::fetch_option_underlying()
            .unwrap_or_else(|_| vec!["BTC-USD".to_string(), "ETH-USD".to_string()]);
        let channels: Vec<String> = underlying
            .into_iter()
            .map(|x| format!("option/trades:{}", x))
            .collect();

        let ws_client = OkexWSClient::new(on_msg_ext, None);
        ws_client.subscribe(&channels);
        ws_client.run(duration);
        None
    } else {
        crawl_trade_internal(market_type, symbols, on_msg, duration)
    }
}

#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event, OkexWSClient, MessageType::L2Event, subscribe_orderbook);

#[rustfmt::skip]
gen_crawl_event!(crawl_bbo, OkexWSClient, MessageType::BBO, subscribe_bbo);

#[rustfmt::skip]
gen_crawl_event!(crawl_l2_topk, OkexWSClient, MessageType::L2TopK, subscribe_orderbook_topk);

#[rustfmt::skip]
gen_crawl_event!(crawl_ticker, OkexWSClient, MessageType::Ticker, subscribe_ticker);

#[allow(clippy::unnecessary_unwrap)]
pub(crate) fn crawl_funding_rate(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) {
    let on_msg_ext = Arc::new(Mutex::new(move |msg: String| {
        let message = Message::new(
            EXCHANGE_NAME.to_string(),
            market_type,
            MessageType::FundingRate,
            msg,
        );
        (on_msg.lock().unwrap())(message);
    }));

    let symbols: Vec<String> = if symbols.is_none() || symbols.unwrap().is_empty() {
        fetch_symbols_retry(EXCHANGE_NAME, market_type)
    } else {
        symbols.unwrap().to_vec()
    };
    let channels: Vec<String> = symbols
        .into_iter()
        .map(|symbol| format!("swap/funding_rate:{}", symbol))
        .collect();

    match market_type {
        MarketType::InverseSwap | MarketType::LinearSwap => {
            let ws_client = OkexWSClient::new(on_msg_ext, None);
            ws_client.subscribe(&channels);
            ws_client.run(duration);
        }
        _ => panic!("OKEx {} does NOT have funding rates", market_type),
    }
}
