use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant, SystemTime},
};

use crate::utils::REST_LOCKS;
use crypto_markets::{fetch_symbols, get_market_types, MarketType};
use crypto_rest_client::{fetch_l2_snapshot, fetch_l3_snapshot};
use log::*;

use crate::{get_hot_spot_symbols, utils::cmc_rank::sort_by_cmc_rank, Message, MessageType};

pub fn fetch_symbols_retry(exchange: &str, market_type: MarketType) -> Vec<String> {
    let retry_count = std::env::var("REST_RETRY_COUNT")
        .unwrap_or_else(|_| "5".to_string())
        .parse::<i64>()
        .unwrap();
    let cooldown_time = get_cooldown_time_per_request(exchange, market_type);
    let lock = REST_LOCKS
        .get(exchange)
        .unwrap()
        .get(&market_type)
        .unwrap()
        .clone();
    let mut symbols = Vec::<String>::new();
    let mut backoff_factor = 1;
    for i in 0..retry_count {
        let mut lock_ = lock.lock().unwrap();
        lock_.lock().unwrap();
        match fetch_symbols(exchange, market_type) {
            Ok(list) => {
                symbols = list;
                lock_.unlock().unwrap();
                break;
            }
            Err(err) => {
                backoff_factor *= 2;
                if i == retry_count - 1 {
                    error!("The {}th time, {}", i, err);
                } else {
                    warn!("The {}th time, {}", i, err);
                }
            }
        }
        // Cooldown after each request, and make all other processes wait
        // on the lock to avoid parallel requests, thus avoid 429 error
        std::thread::sleep(cooldown_time * backoff_factor);
        lock_.unlock().unwrap();
    }
    symbols
}

pub(super) fn check_args(exchange: &str, market_type: MarketType, symbols: &[String]) {
    let market_types = get_market_types(exchange);
    if !market_types.contains(&market_type) {
        panic!("{} does NOT have the {} market type", exchange, market_type);
    }

    let valid_symbols = fetch_symbols_retry(exchange, market_type);
    let invalid_symbols: Vec<String> = symbols
        .iter()
        // at Binance all symbols for websocket are lowercase while for REST they are uppercase
        .map(|symbol| {
            if exchange == "binance" {
                symbol.to_uppercase()
            } else {
                symbol.to_string()
            }
        })
        .filter(|symbol| !valid_symbols.contains(symbol))
        .collect();
    if !invalid_symbols.is_empty() {
        panic!(
            "Invalid symbols: {}, {} {} available trading symbols are {}",
            invalid_symbols.join(","),
            exchange,
            market_type,
            valid_symbols.join(",")
        );
    }
}

fn get_cooldown_time_per_request(exchange: &str, market_type: MarketType) -> Duration {
    let millis = match exchange {
        "binance" => 500,      // spot weitht 1200, contract weight 2400
        "bitget" => 100,       // 20 requests per 2 seconds
        "bithumb" => 8 * 10, // 135 requests per 1 second for public APIs, multiplied by 10 to reduce its frequency
        "bitmex" => 2000, // 60 requests per minute on all routes (reduced to 30 when unauthenticated)
        "bitstamp" => 75 * 10, // 8000 requests per 10 minutes, but bitstamp orderbook is too big, need to reduce its frequency
        "bitz" => 34,          // no more than 30 times within 1 second
        "bybit" => 20 * 10, // 50 requests per second continuously for 2 minutes, multiplied by 10 to reduce its frequency
        "coinbase_pro" => 100, //  10 requests per second
        "deribit" => 50,    // 20 requests per second
        "gate" => 4,        // 300 read operations per IP per second
        "huobi" => 2,       // 800 times/second for one IP
        "kucoin" => match market_type {
            MarketType::Spot => 200, // 2x to avoid 429
            _ => 100,                // 30 times/3s
        },
        "mxc" => 100,  // 20 times per 2 seconds
        "okex" => 100, // 20 requests per 2 seconds
        _ => 100,
    };
    Duration::from_millis(millis)
}

/// Crawl leve2 or level3 orderbook snapshots through RESTful APIs.
pub(crate) fn crawl_snapshot(
    exchange: &str,
    market_type: MarketType,
    msg_type: MessageType, // L2Snapshot or L3Snapshot
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) {
    let now = Instant::now();
    let is_empty = match symbols {
        Some(list) => {
            if list.is_empty() {
                true
            } else {
                check_args(exchange, market_type, list);
                false
            }
        }
        None => true,
    };

    let cooldown_time = get_cooldown_time_per_request(exchange, market_type);

    let lock = REST_LOCKS
        .get(exchange)
        .unwrap()
        .get(&market_type)
        .unwrap()
        .clone();
    loop {
        let mut real_symbols = if is_empty {
            if market_type == MarketType::Spot {
                let spot_symbols = fetch_symbols_retry(exchange, market_type);
                get_hot_spot_symbols(exchange, &spot_symbols)
            } else {
                fetch_symbols_retry(exchange, market_type)
            }
        } else {
            symbols.unwrap().to_vec()
        };
        sort_by_cmc_rank(exchange, &mut real_symbols);

        let mut index = 0_usize;
        let mut success_count = 0_u64;
        let mut backoff_factor = 1;
        while index < real_symbols.len() {
            let symbol = &real_symbols[index];
            let mut lock_ = lock.lock().unwrap();
            lock_.lock().unwrap();
            let resp = match msg_type {
                MessageType::L2Snapshot => fetch_l2_snapshot(exchange, market_type, symbol, None),
                MessageType::L3Snapshot => fetch_l3_snapshot(exchange, market_type, symbol, None),
                _ => panic!("msg_type must be L2Snapshot or L3Snapshot"),
            };
            // Cooldown after each request, and make all other processes wait
            // on the lock to avoid parallel requests, thus avoid 429 error
            std::thread::sleep(cooldown_time);
            lock_.unlock().unwrap();
            match resp {
                Ok(msg) => {
                    index += 1;
                    success_count += 1;
                    backoff_factor = 1;
                    let message = Message::new(exchange.to_string(), market_type, msg_type, msg);
                    (on_msg.lock().unwrap())(message);
                }
                Err(err) => {
                    let current_timestamp = SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64;
                    warn!(
                        "{} {} {} {} {} {}, error: {}, back off for {} milliseconds",
                        current_timestamp,
                        success_count,
                        backoff_factor,
                        exchange,
                        market_type,
                        symbol,
                        err,
                        (backoff_factor * cooldown_time).as_millis()
                    );
                    std::thread::sleep(backoff_factor * cooldown_time);
                    success_count = 0;
                    if err.0.contains("429") || err.0.contains("509") {
                        backoff_factor += 1;
                    } else {
                        // Handle 403, 418, etc.
                        backoff_factor *= 2;
                    }
                }
            }
        }
        if let Some(seconds) = duration {
            if now.elapsed() > Duration::from_secs(seconds) {
                break;
            }
        }
        std::thread::sleep(cooldown_time * 2); // if real_symbols is empty, CPU will be 100% without this line
    }
}

macro_rules! subscribe_with_lock {
    ($ws_client:ident, $exchange:expr, $market_type: expr, $subscribe_func:ident, $symbols:expr, $lock:ident) => {{
        let mut lock = $lock.lock().unwrap();
        let interval = get_send_interval_ms($exchange, $market_type);
        if interval.is_some() {
            lock.lock().unwrap();
        }
        $ws_client.$subscribe_func($symbols);
        if let Some(interval) = interval {
            std::thread::sleep(Duration::from_millis(interval));
            lock.unlock().unwrap();
        }
    }};
}

macro_rules! gen_crawl_event {
    ($func_name:ident, $struct_name:ident, $msg_type:expr, $subscribe_func:ident) => {
        pub(crate) fn $func_name(
            market_type: MarketType,
            symbols: Option<&[String]>,
            on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
            duration: Option<u64>,
        ) -> Option<std::thread::JoinHandle<()>> {
            let is_empty = match symbols {
                Some(list) => {
                    if list.is_empty() {
                        true
                    } else {
                        check_args(EXCHANGE_NAME, market_type, &list);
                        false
                    }
                }
                None => true,
            };
            let automatic_symbol_discovery = is_empty && duration.is_none();

            let real_symbols = if is_empty {
                if EXCHANGE_NAME == "binance" {
                    fetch_symbols_retry(EXCHANGE_NAME, market_type)
                        .into_iter()
                        .map(|s| s.to_lowercase())
                        .collect()
                } else {
                    fetch_symbols_retry(EXCHANGE_NAME, market_type)
                }
            } else {
                symbols.unwrap().iter().cloned().collect::<Vec<String>>()
            };
            if real_symbols.is_empty() {
                panic!("real_symbols is empty");
            }

            let on_msg_ext = Arc::new(Mutex::new(move |msg: String| {
                let message = Message::new(
                    EXCHANGE_NAME.to_string(),
                    market_type,
                    $msg_type,
                    msg.to_string(),
                );
                (on_msg.lock().unwrap())(message);
            }));

            let lock = WS_LOCKS
                .get(EXCHANGE_NAME)
                .unwrap()
                .get(&market_type)
                .unwrap()
                .clone();

            let on_msg_ext_clone = on_msg_ext.clone();
            let mut subscribed_symbols = real_symbols.clone();
            let lock_clone = lock.clone();
            let create_symbol_discovery_thread = move || -> std::thread::JoinHandle<()> {
                std::thread::spawn(move || {
                    let ws_client = {
                        let mut lock_ = lock_clone.lock().unwrap();
                        let interval = get_connection_interval_ms(EXCHANGE_NAME, market_type);
                        if interval.is_some() {
                            lock_.lock().unwrap();
                        }
                        let ws_client = Arc::new($struct_name::new(on_msg_ext_clone, None));
                        if let Some(interval) = interval {
                            std::thread::sleep(Duration::from_millis(interval));
                            lock_.unlock().unwrap();
                        }
                        ws_client
                    };
                    let ws_client_clone = ws_client.clone();
                    let should_stop = Arc::new(AtomicBool::new(false));
                    let should_stop_clone = should_stop.clone();
                    std::thread::spawn(move || {
                        let mut total_new_symbols = 0;
                        while !should_stop_clone.load(Ordering::Acquire) {
                            // update symbols every hour
                            std::thread::sleep(Duration::from_secs(3600));
                            let latest_symbols = if EXCHANGE_NAME == "binance" {
                                fetch_symbols_retry(EXCHANGE_NAME, market_type)
                                    .into_iter()
                                    .map(|s| s.to_lowercase())
                                    .collect()
                            } else {
                                fetch_symbols_retry(EXCHANGE_NAME, market_type)
                            };
                            let mut new_symbols: Vec<String> = latest_symbols
                                .iter()
                                .filter(|s| !subscribed_symbols.contains(s))
                                .cloned()
                                .collect();

                            if !new_symbols.is_empty() {
                                warn!("Found new symbols: {}", new_symbols.join(", "));
                                subscribe_with_lock!(
                                    ws_client_clone,
                                    EXCHANGE_NAME,
                                    market_type,
                                    $subscribe_func,
                                    &new_symbols,
                                    lock_clone
                                );
                                subscribed_symbols.append(&mut new_symbols);
                            }
                            total_new_symbols += new_symbols.len();
                            if total_new_symbols > MAX_SUBSCRIPTIONS_PER_CONNECTION {
                                warn!(
                                    "Found {} new symbols, restarting the process",
                                    new_symbols.len()
                                );
                                std::process::exit(0); // restart the whole process
                            }
                        }
                    });
                    ws_client.run(duration);
                    ws_client.close();
                    should_stop.store(true, Ordering::Release);
                })
            };

            let thread = if automatic_symbol_discovery {
                Some(create_symbol_discovery_thread())
            } else {
                None
            };

            if real_symbols.len() <= MAX_SUBSCRIPTIONS_PER_CONNECTION {
                let lock_clone = lock.clone();
                let on_msg_ext_clone = on_msg_ext.clone();
                let ws_client = {
                    let mut lock_ = lock_clone.lock().unwrap();
                    let interval = get_connection_interval_ms(EXCHANGE_NAME, market_type);
                    if interval.is_some() {
                        lock_.lock().unwrap();
                    }
                    let ws_client = Arc::new($struct_name::new(on_msg_ext_clone, None));
                    if let Some(interval) = interval {
                        std::thread::sleep(Duration::from_millis(interval));
                        lock_.unlock().unwrap();
                    }
                    ws_client
                };
                subscribe_with_lock!(
                    ws_client,
                    EXCHANGE_NAME,
                    market_type,
                    $subscribe_func,
                    &real_symbols,
                    lock_clone
                );
                drop(lock);
                ws_client.run(duration);
                ws_client.close();
            } else {
                // split to chunks
                let mut chunks: Vec<Vec<String>> = Vec::new();
                for i in (0..real_symbols.len()).step_by(MAX_SUBSCRIPTIONS_PER_CONNECTION) {
                    let chunk = (&real_symbols[i..(std::cmp::min(
                        i + MAX_SUBSCRIPTIONS_PER_CONNECTION,
                        real_symbols.len(),
                    ))])
                        .iter()
                        .cloned()
                        .collect();
                    chunks.push(chunk);
                }
                assert!(chunks.len() > 1);

                let mut join_handles: Vec<std::thread::JoinHandle<()>> = Vec::new();
                for chunk in chunks.into_iter() {
                    let on_msg_ext_clone = on_msg_ext.clone();
                    let lock_clone = lock.clone();
                    let handle = std::thread::spawn(move || {
                        let ws_client = {
                            let mut lock_ = lock_clone.lock().unwrap();
                            let interval = get_connection_interval_ms(EXCHANGE_NAME, market_type);
                            if interval.is_some() {
                                lock_.lock().unwrap();
                            }
                            let ws_client = Arc::new($struct_name::new(on_msg_ext_clone, None));
                            if let Some(interval) = interval {
                                std::thread::sleep(Duration::from_millis(interval));
                                lock_.unlock().unwrap();
                            }
                            ws_client
                        };
                        subscribe_with_lock!(
                            ws_client,
                            EXCHANGE_NAME,
                            market_type,
                            $subscribe_func,
                            &chunk,
                            lock_clone
                        );
                        ws_client.run(duration);
                        ws_client.close();
                    });
                    join_handles.push(handle);
                }
                for handle in join_handles {
                    handle.join().unwrap();
                }
            }
            thread
        }
    };
}

// from 1m to 5m
pub(crate) fn get_candlestick_intervals(exchange: &str, market_type: MarketType) -> Vec<usize> {
    match exchange {
        "binance" => vec![60, 180, 300],
        "bybit" => vec![60, 180, 300],
        "deribit" => vec![60, 180, 300],
        "gate" => vec![10, 60, 300],
        "kucoin" => match market_type {
            MarketType::Spot => vec![60, 180, 300],
            _ => vec![60, 300],
        },
        "okex" => vec![60, 180, 300],
        "zbg" => match market_type {
            MarketType::Spot => vec![60, 300],
            _ => vec![60, 180, 300],
        },
        _ => vec![60, 300],
    }
}

pub(crate) fn get_connection_interval_ms(exchange: &str, _market_type: MarketType) -> Option<u64> {
    match exchange {
        // "bitmex" => Some(9000), // 40 per hour
        "kucoin" => Some(2000), //  Connection Limit: 30 per minute
        "okex" => Some(1000),   //  Connection limit：1 times/s
        _ => Some(20),
    }
}

pub(crate) fn get_send_interval_ms(exchange: &str, _market_type: MarketType) -> Option<u64> {
    match exchange {
        "binance" => Some(100), // WebSocket connections have a limit of 10 incoming messages per second
        "kucoin" => Some(100),  //  Message limit sent to the server: 100 per 10 seconds
        // "okex" => Some(15000), // 240 times/hour
        _ => None,
    }
}

macro_rules! gen_crawl_candlestick {
    ($func_name:ident, $struct_name:ident) => {
        pub(crate) fn $func_name(
            market_type: MarketType,
            symbol_interval_list: Option<&[(String, usize)]>,
            on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
            duration: Option<u64>,
        ) -> Option<std::thread::JoinHandle<()>> {
            let is_empty = match symbol_interval_list {
                Some(list) => {
                    if list.is_empty() {
                        true
                    } else {
                        let symbols: Vec<String> = list.iter().map(|t| t.0.clone()).collect();
                        check_args(EXCHANGE_NAME, market_type, symbols.as_slice());
                        false
                    }
                }
                None => true,
            };
            let automatic_symbol_discovery = is_empty && duration.is_none();

            let symbol_interval_list: Vec<(String, usize)> = if is_empty {
                let symbols = if EXCHANGE_NAME == "binance" {
                    fetch_symbols_retry(EXCHANGE_NAME, market_type)
                        .into_iter()
                        .map(|s| s.to_lowercase())
                        .collect()
                } else {
                    fetch_symbols_retry(EXCHANGE_NAME, market_type)
                };
                let intervals = get_candlestick_intervals(EXCHANGE_NAME, market_type);
                symbols
                    .iter()
                    .flat_map(|symbol| {
                        intervals
                            .clone()
                            .into_iter()
                            .map(move |interval| (symbol.clone(), interval))
                    })
                    .collect::<Vec<(String, usize)>>()
            } else {
                symbol_interval_list
                    .unwrap()
                    .iter()
                    .cloned()
                    .collect::<Vec<(String, usize)>>()
            };
            if symbol_interval_list.is_empty() {
                panic!("symbol_interval_list is empty");
            }
            let real_symbols: Vec<String> =
                symbol_interval_list.iter().map(|t| t.0.clone()).collect();
            let real_intervals: Vec<usize> = symbol_interval_list.iter().map(|t| t.1).collect();

            let on_msg_ext = Arc::new(Mutex::new(move |msg: String| {
                let message = Message::new(
                    EXCHANGE_NAME.to_string(),
                    market_type,
                    MessageType::Candlestick,
                    msg.to_string(),
                );
                (on_msg.lock().unwrap())(message);
            }));

            let lock = WS_LOCKS
                .get(EXCHANGE_NAME)
                .unwrap()
                .get(&market_type)
                .unwrap()
                .clone();

            let thread = if automatic_symbol_discovery {
                let on_msg_ext_clone = on_msg_ext.clone();
                let intervals_clone = real_intervals.clone();
                let mut subscribed_symbols = real_symbols.clone();
                let lock_clone = lock.clone();
                let thread = std::thread::spawn(move || {
                    let ws_client = {
                        let mut lock_ = lock_clone.lock().unwrap();
                        let interval = get_connection_interval_ms(EXCHANGE_NAME, market_type);
                        if interval.is_some() {
                            lock_.lock().unwrap();
                        }
                        let ws_client = Arc::new($struct_name::new(on_msg_ext_clone, None));
                        if let Some(interval) = interval {
                            std::thread::sleep(Duration::from_millis(interval));
                            lock_.unlock().unwrap();
                        }
                        ws_client
                    };
                    let ws_client_clone = ws_client.clone();
                    let should_stop = Arc::new(AtomicBool::new(false));
                    let should_stop_clone = should_stop.clone();
                    std::thread::spawn(move || {
                        let mut total_new_symbols = 0;
                        while !should_stop_clone.load(Ordering::Acquire) {
                            // update symbols every hour
                            std::thread::sleep(Duration::from_secs(3600));
                            let latest_symbols = if EXCHANGE_NAME == "binance" {
                                fetch_symbols_retry(EXCHANGE_NAME, market_type)
                                    .into_iter()
                                    .map(|s| s.to_lowercase())
                                    .collect()
                            } else {
                                fetch_symbols_retry(EXCHANGE_NAME, market_type)
                            };
                            let mut new_symbols: Vec<String> = latest_symbols
                                .iter()
                                .filter(|s| !subscribed_symbols.contains(s))
                                .cloned()
                                .collect();
                            if !new_symbols.is_empty() {
                                warn!("Found new symbols: {}", new_symbols.join(", "));
                                let new_symbol_interval_list = new_symbols
                                    .iter()
                                    .flat_map(|symbol| {
                                        intervals_clone
                                            .clone()
                                            .into_iter()
                                            .map(move |interval| (symbol.clone(), interval))
                                    })
                                    .collect::<Vec<(String, usize)>>();
                                subscribe_with_lock!(
                                    ws_client_clone,
                                    EXCHANGE_NAME,
                                    market_type,
                                    subscribe_candlestick,
                                    new_symbol_interval_list.as_slice(),
                                    lock_clone
                                );
                                subscribed_symbols.append(&mut new_symbols);
                            }
                            total_new_symbols += new_symbols.len();
                            if total_new_symbols > MAX_SUBSCRIPTIONS_PER_CONNECTION {
                                warn!(
                                    "Found {} new symbols, restarting the process",
                                    new_symbols.len()
                                );
                                std::process::exit(0); // restart the whole process
                            }
                        }
                    });
                    ws_client.run(duration);
                    ws_client.close();
                    should_stop.store(true, Ordering::Release);
                });
                Some(thread)
            } else {
                None
            };

            if symbol_interval_list.len() <= MAX_SUBSCRIPTIONS_PER_CONNECTION {
                let lock_clone = lock.clone();
                let on_msg_ext_clone = on_msg_ext.clone();
                let ws_client = {
                    let mut lock_ = lock_clone.lock().unwrap();
                    let interval = get_connection_interval_ms(EXCHANGE_NAME, market_type);
                    if interval.is_some() {
                        lock_.lock().unwrap();
                    }
                    let ws_client = Arc::new($struct_name::new(on_msg_ext_clone, None));
                    if let Some(interval) = interval {
                        std::thread::sleep(Duration::from_millis(interval));
                        lock_.unlock().unwrap();
                    }
                    ws_client
                };
                subscribe_with_lock!(
                    ws_client,
                    EXCHANGE_NAME,
                    market_type,
                    subscribe_candlestick,
                    symbol_interval_list.as_slice(),
                    lock_clone
                );
                drop(lock);
                ws_client.run(duration);
                ws_client.close();
            } else {
                // split to chunks
                let mut chunks: Vec<Vec<(String, usize)>> = Vec::new();
                for i in (0..symbol_interval_list.len()).step_by(MAX_SUBSCRIPTIONS_PER_CONNECTION) {
                    let chunk: Vec<(String, usize)> = (&symbol_interval_list[i..(std::cmp::min(
                        i + MAX_SUBSCRIPTIONS_PER_CONNECTION,
                        symbol_interval_list.len(),
                    ))])
                        .iter()
                        .cloned()
                        .collect();
                    chunks.push(chunk);
                }
                assert!(chunks.len() > 1);

                let mut join_handles: Vec<std::thread::JoinHandle<()>> = Vec::new();
                for chunk in chunks.into_iter() {
                    let on_msg_ext_clone = on_msg_ext.clone();
                    let lock_clone = lock.clone();
                    let handle = std::thread::spawn(move || {
                        let ws_client = {
                            let mut lock_ = lock_clone.lock().unwrap();
                            let interval = get_connection_interval_ms(EXCHANGE_NAME, market_type);
                            if interval.is_some() {
                                lock_.lock().unwrap();
                            }
                            let ws_client = Arc::new($struct_name::new(on_msg_ext_clone, None));
                            if let Some(interval) = interval {
                                std::thread::sleep(Duration::from_millis(interval));
                                lock_.unlock().unwrap();
                            }
                            ws_client
                        };
                        subscribe_with_lock!(
                            ws_client,
                            EXCHANGE_NAME,
                            market_type,
                            subscribe_candlestick,
                            chunk.as_slice(),
                            lock_clone
                        );
                        ws_client.run(duration);
                        ws_client.close();
                    });
                    join_handles.push(handle);
                }
                for handle in join_handles {
                    handle.join().unwrap();
                }
            }
            thread
        }
    };
}
