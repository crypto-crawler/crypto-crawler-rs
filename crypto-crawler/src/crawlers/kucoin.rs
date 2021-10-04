use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use std::time::Duration;

use super::utils::{check_args, fetch_symbols_retry};
use crate::{msg::Message, MessageType};
use crypto_markets::MarketType;
use crypto_ws_client::*;
use log::*;

const EXCHANGE_NAME: &str = "kucoin";
// See https://docs.kucoin.cc/#request-rate-limit
const MAX_SUBSCRIPTIONS_PER_CONNECTION: usize = 300;

#[rustfmt::skip]
gen_crawl_event!(crawl_trade_spot, KuCoinSpotWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_swap, KuCoinSwapWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]

#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_spot, KuCoinSpotWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_swap, KuCoinSwapWSClient, MessageType::L2Event, subscribe_orderbook);

#[rustfmt::skip]
gen_crawl_event!(crawl_bbo_spot, KuCoinSpotWSClient, MessageType::BBO, subscribe_bbo);
#[rustfmt::skip]
gen_crawl_event!(crawl_bbo_swap, KuCoinSwapWSClient, MessageType::BBO, subscribe_bbo);

#[rustfmt::skip]
gen_crawl_event!(crawl_l2_topk_spot, KuCoinSpotWSClient, MessageType::L2TopK, subscribe_orderbook_topk);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_topk_swap, KuCoinSwapWSClient, MessageType::L2TopK, subscribe_orderbook_topk);

#[rustfmt::skip]
gen_crawl_event!(crawl_l3_event_spot, KuCoinSpotWSClient, MessageType::L3Event, subscribe_l3_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_l3_event_swap, KuCoinSwapWSClient, MessageType::L3Event, subscribe_l3_orderbook);

#[rustfmt::skip]
gen_crawl_event!(crawl_ticker_spot, KuCoinSpotWSClient, MessageType::Ticker, subscribe_ticker);
#[rustfmt::skip]
gen_crawl_event!(crawl_ticker_swap, KuCoinSwapWSClient, MessageType::Ticker, subscribe_ticker);
#[rustfmt::skip]

pub(crate) fn crawl_trade(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::Spot => crawl_trade_spot(market_type, symbols, on_msg, duration),
        MarketType::InverseSwap | MarketType::LinearSwap | MarketType::InverseFuture => {
            crawl_trade_swap(market_type, symbols, on_msg, duration)
        }
        _ => panic!("KuCoin does NOT have the {} market type", market_type),
    }
}

pub(crate) fn crawl_l2_event(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::Spot => crawl_l2_event_spot(market_type, symbols, on_msg, duration),
        MarketType::InverseSwap | MarketType::LinearSwap | MarketType::InverseFuture => {
            crawl_l2_event_swap(market_type, symbols, on_msg, duration)
        }
        _ => panic!("KuCoin does NOT have the {} market type", market_type),
    }
}

pub(crate) fn crawl_bbo(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::Spot => {
            if symbols.is_none() || symbols.unwrap().is_empty() {
                let on_msg_ext = Arc::new(Mutex::new(move |msg: String| {
                    let message = Message::new(
                        EXCHANGE_NAME.to_string(),
                        market_type,
                        MessageType::BBO,
                        msg,
                    );
                    (on_msg.lock().unwrap())(message);
                }));

                // https://docs.kucoin.com/#all-symbols-ticker
                let channels: Vec<String> = vec!["/market/ticker:all".to_string()];

                let ws_client = KuCoinSpotWSClient::new(on_msg_ext, None);
                ws_client.subscribe(&channels);
                ws_client.run(duration);
                None
            } else {
                crawl_bbo_spot(market_type, symbols, on_msg, duration)
            }
        }
        MarketType::InverseSwap | MarketType::LinearSwap | MarketType::InverseFuture => {
            crawl_bbo_swap(market_type, symbols, on_msg, duration)
        }
        _ => panic!("KuCoin does NOT have the {} market type", market_type),
    }
}

pub(crate) fn crawl_l2_topk(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::Spot => crawl_l2_topk_spot(market_type, symbols, on_msg, duration),
        MarketType::InverseSwap | MarketType::LinearSwap | MarketType::InverseFuture => {
            crawl_l2_topk_swap(market_type, symbols, on_msg, duration)
        }
        _ => panic!("KuCoin does NOT have the {} market type", market_type),
    }
}

pub(crate) fn crawl_l3_event(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::Spot => crawl_l3_event_spot(market_type, symbols, on_msg, duration),
        MarketType::InverseSwap | MarketType::LinearSwap | MarketType::InverseFuture => {
            crawl_l3_event_swap(market_type, symbols, on_msg, duration)
        }
        _ => panic!("KuCoin does NOT have the {} market type", market_type),
    }
}

pub(crate) fn crawl_ticker(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    match market_type {
        MarketType::Spot => crawl_ticker_spot(market_type, symbols, on_msg, duration),
        MarketType::InverseSwap | MarketType::LinearSwap | MarketType::InverseFuture => {
            crawl_ticker_swap(market_type, symbols, on_msg, duration)
        }
        _ => panic!("KuCoin does NOT have the {} market type", market_type),
    }
}
