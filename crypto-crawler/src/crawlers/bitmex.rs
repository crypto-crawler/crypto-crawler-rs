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

const EXCHANGE_NAME: &str = "bitmex";

// see <https://www.bitmex.com/app/wsAPI#Rate-Limits>
const MAX_SUBSCRIPTIONS_PER_CONNECTION: usize = 40;

#[rustfmt::skip]
gen_crawl_event!(crawl_trade, BitmexWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event, BitmexWSClient, MessageType::L2Event, subscribe_orderbook);

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
        .map(|symbol| format!("funding:{}", symbol))
        .collect();

    match market_type {
        MarketType::InverseSwap | MarketType::QuantoSwap => {
            let ws_client = BitmexWSClient::new(on_msg_ext, None);
            ws_client.subscribe(&channels);
            ws_client.run(duration);
        }
        _ => panic!("BitMEX {} does NOT have funding rates", market_type),
    }
}
