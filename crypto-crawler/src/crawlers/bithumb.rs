use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use std::time::Duration;

use super::utils::{
    check_args, fetch_symbols_retry, get_connection_interval_ms, get_send_interval_ms,
};
use crate::utils::WS_LOCKS;
use crate::{msg::Message, MessageType};
use crypto_markets::MarketType;
use crypto_ws_client::*;
use log::*;

const EXCHANGE_NAME: &str = "bithumb";
// usize::MAX means unlimited
const MAX_SUBSCRIPTIONS_PER_CONNECTION: usize = usize::MAX;

#[rustfmt::skip]
gen_crawl_event!(crawl_trade, BithumbWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event, BithumbWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_ticker, BithumbWSClient, MessageType::Ticker, subscribe_ticker);
