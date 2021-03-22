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
