use std::{cell::RefCell, rc::Rc};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::{msg::Message, MarketType, MessageType};
use crypto_rest_client::*;
use crypto_ws_client::*;
use log::*;
use serde_json::Value;

const EXCHANGE_NAME: &str = "Huobi";

fn extract_symbol(json: &str) -> String {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&json).unwrap();
    let channel = obj.get("ch").unwrap().as_str().unwrap();
    let symbol = channel.split('.').nth(1).unwrap();
    symbol.to_string()
}

fn check_args(_market_type: MarketType, _symbols: &[String]) {
    // TODO: add more check
}

#[rustfmt::skip]
gen_crawl_event!(crawl_trade_spot, market_type, symbols, on_msg, duration, HuobiSpotWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_future, market_type, symbols, on_msg, duration, HuobiFutureWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_future, market_type, symbols, on_msg, duration, HuobiFutureWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_linear_swap, market_type, symbols, on_msg, duration, HuobiLinearSwapWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_linear_swap, market_type, symbols, on_msg, duration, HuobiLinearSwapWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_inverse_swap, market_type, symbols, on_msg, duration, HuobiInverseSwapWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_inverse_swap, market_type, symbols, on_msg, duration, HuobiInverseSwapWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_trade_option, market_type, symbols, on_msg, duration, HuobiOptionWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event_option, market_type, symbols, on_msg, duration, HuobiOptionWSClient, MessageType::L2Event, subscribe_orderbook);

pub(crate) fn crawl_trade<'a>(
    market_type: MarketType,
    symbols: &[String],
    on_msg: Rc<RefCell<dyn FnMut(Message) + 'a>>,
    duration: Option<u64>,
) {
    check_args(market_type, symbols);
    match market_type {
        MarketType::Spot => crawl_trade_spot(market_type, symbols, on_msg, duration),
        MarketType::Future => crawl_trade_future(market_type, symbols, on_msg, duration),
        MarketType::Swap => {
            if symbols[0].ends_with("-USDT") {
                crawl_trade_linear_swap(market_type, symbols, on_msg, duration);
            } else {
                crawl_trade_inverse_swap(market_type, symbols, on_msg, duration);
            }
        }
        MarketType::Option => crawl_trade_option(market_type, symbols, on_msg, duration),
    }
}

pub(crate) fn crawl_l2_event<'a>(
    market_type: MarketType,
    symbols: &[String],
    on_msg: Rc<RefCell<dyn FnMut(Message) + 'a>>,
    duration: Option<u64>,
) {
    check_args(market_type, symbols);
    match market_type {
        MarketType::Spot => {
            check_args(market_type, symbols);
            let on_msg_ext = |msg: String| {
                let message = Message::new(
                    EXCHANGE_NAME.to_string(),
                    market_type,
                    extract_symbol(&msg),
                    MessageType::L2Event,
                    msg.to_string(),
                );
                (on_msg.borrow_mut())(message);
            };
            // Huobi Spot market.$symbol.mbp.$levels must use wss://api.huobi.pro/feed
            // or wss://api-aws.huobi.pro/feed
            let mut ws_client = HuobiSpotWSClient::new(
                Rc::new(RefCell::new(on_msg_ext)),
                Some("wss://api.huobi.pro/feed"),
            );
            ws_client.subscribe_orderbook(symbols);
            ws_client.run(duration);
        }
        MarketType::Future => crawl_l2_event_future(market_type, symbols, on_msg, duration),
        MarketType::Swap => {
            if symbols[0].ends_with("-USDT") {
                crawl_l2_event_linear_swap(market_type, symbols, on_msg, duration);
            } else {
                crawl_l2_event_inverse_swap(market_type, symbols, on_msg, duration);
            }
        }
        MarketType::Option => crawl_l2_event_option(market_type, symbols, on_msg, duration),
    }
}

pub(crate) fn crawl_l2_snapshot<'a>(
    market_type: MarketType,
    symbols: &[String],
    on_msg: Rc<RefCell<dyn FnMut(Message) + 'a>>,
    duration: Option<u64>,
) {
    let on_msg_ext = |json: String, symbol: String| {
        let message = Message::new(
            EXCHANGE_NAME.to_string(),
            market_type,
            symbol,
            MessageType::L2Snapshot,
            json,
        );
        (on_msg.borrow_mut())(message);
    };

    let now = Instant::now();
    loop {
        let mut succeeded = false;
        for symbol in symbols.iter() {
            let resp = match market_type {
                MarketType::Spot => HuobiSpotRestClient::fetch_l2_snapshot(symbol, 1),
                MarketType::Future => HuobiFutureRestClient::fetch_l2_snapshot(symbol, 1),
                MarketType::Swap => {
                    if symbol.ends_with("-USDT") {
                        HuobiLinearSwapRestClient::fetch_l2_snapshot(symbol, 1)
                    } else {
                        HuobiInverseSwapRestClient::fetch_l2_snapshot(symbol, 1)
                    }
                }
                MarketType::Option => HuobiOptionRestClient::fetch_l2_snapshot(symbol, 1),
            };
            match resp {
                Ok(msg) => {
                    on_msg_ext(msg, symbol.to_string());
                    succeeded = true
                }
                Err(err) => error!(
                    "{} {} {}, error: {}",
                    EXCHANGE_NAME, market_type, symbol, err
                ),
            }
        }

        if let Some(seconds) = duration {
            if now.elapsed() > Duration::from_secs(seconds) && succeeded {
                break;
            }
        }

        std::thread::sleep(Duration::from_secs(crate::SNAPSHOT_INTERVAL));
    }
}
