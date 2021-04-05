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

const EXCHANGE_NAME: &str = "kraken";
// usize::MAX means unlimited
const MAX_SUBSCRIPTIONS_PER_CONNECTION: usize = usize::MAX;

#[rustfmt::skip]
gen_crawl_event!(crawl_trade, KrakenWSClient, MessageType::Trade, subscribe_trade);
#[rustfmt::skip]
gen_crawl_event!(crawl_l2_event, KrakenWSClient, MessageType::L2Event, subscribe_orderbook);
#[rustfmt::skip]
gen_crawl_event!(crawl_ticker, KrakenWSClient, MessageType::Ticker, subscribe_ticker);
