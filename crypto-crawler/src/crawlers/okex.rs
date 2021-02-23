use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc, Mutex,
};

use std::{collections::HashMap, time::Duration};

use super::utils::{check_args, fetch_symbols_retry};
use crate::{msg::Message, MessageType};
use crypto_markets::MarketType;
use crypto_rest_client::*;
use crypto_ws_client::*;
use log::*;
use serde_json::Value;

const EXCHANGE_NAME: &str = "okex";
// usize::MAX means unlimited
const MAX_SUBSCRIPTIONS_PER_CONNECTION: usize = usize::MAX;

fn extract_symbol(json: &str) -> String {
    let obj = serde_json::from_str::<HashMap<String, Value>>(&json).unwrap();
    let arr = obj.get("data").unwrap().as_array().unwrap();
    debug_assert_eq!(1, arr.len());
    let symbol = arr[0]
        .as_object()
        .unwrap()
        .get("instrument_id")
        .unwrap()
        .as_str()
        .unwrap();
    symbol.to_string()
}

#[rustfmt::skip]
gen_crawl_event!(crawl_trade_internal, OkexWSClient, MessageType::Trade, subscribe_trade);

pub(crate) fn crawl_trade(
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    if market_type == MarketType::Option && (symbols.is_none() || symbols.unwrap().is_empty()) {
        let on_msg_ext = Arc::new(Mutex::new(move |msg: String| {
            let message = Message::new(
                EXCHANGE_NAME.to_string(),
                market_type,
                extract_symbol(&msg),
                MessageType::Trade,
                msg.to_string(),
            );
            (on_msg.lock().unwrap())(message);
        }));

        let underlying = OkexRestClient::fetch_option_underlying()
            .unwrap_or(vec!["BTC-USD".to_string(), "ETH-USD".to_string()]);
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
