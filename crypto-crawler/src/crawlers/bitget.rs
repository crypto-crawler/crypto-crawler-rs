use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc, Mutex,
};

use std::time::Duration;

use super::utils::{check_args, fetch_symbols_retry};
use crate::{msg::Message, MessageType};
use crypto_markets::MarketType;
use crypto_ws_client::*;
use log::*;

const EXCHANGE_NAME: &str = "bitget";
// usize::MAX means unlimited
const MAX_SUBSCRIPTIONS_PER_CONNECTION: usize = usize::MAX;

#[rustfmt::skip]
gen_crawl_event!(crawl_trade_swap, BitgetSwapWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_swap, BitgetSwapWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_ticker_swap, BitgetSwapWSClient, MessageType::Ticker, subscribe_ticker);

pub(crate) fn crawl_trade(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::InverseSwap | MarketType::LinearSwap => {
            crawl_trade_swap(market_type, symbols, on_msg, duration)
        }
        _ => panic!("Bitget does NOT have the {} market type", market_type),
    }
}

pub(crate) fn crawl_l2_event(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::InverseSwap | MarketType::LinearSwap => {
            crawl_l2_event_swap(market_type, symbols, on_msg, duration)
        }
        _ => panic!("Bitget does NOT have the {} market type", market_type),
    }
}

pub(crate) fn crawl_ticker(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::InverseSwap | MarketType::LinearSwap => {
            crawl_ticker_swap(market_type, symbols, on_msg, duration)
        }
        _ => panic!("Bitget does NOT have the {} market type", market_type),
    }
}

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
        symbols
            .unwrap()
            .into_iter()
            .map(|symbol| symbol.to_string())
            .collect()
    };
    let channels: Vec<String> = symbols
        .into_iter()
        .map(|symbol| format!("swap/funding_rate:{}", symbol))
        .collect();

    match market_type {
        MarketType::InverseSwap | MarketType::LinearSwap => {
            let ws_client = BitgetSwapWSClient::new(on_msg_ext, None);
            ws_client.subscribe(&channels);
            ws_client.run(duration);
        }
        _ => panic!("Bitget {} does NOT have funding rates", market_type),
    }
}
