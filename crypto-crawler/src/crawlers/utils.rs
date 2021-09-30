use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant, SystemTime},
};

use crypto_markets::{fetch_symbols, get_market_types, MarketType};
use crypto_rest_client::{fetch_l2_snapshot, fetch_l3_snapshot};
use log::*;

use crate::{
    utils::{cmc_rank::sort_by_cmc_rank, get_hot_spot_symbols},
    Message, MessageType,
};

pub fn fetch_symbols_retry(exchange: &str, market_type: MarketType) -> Vec<String> {
    let retry_count = std::env::var("REST_RETRY_COUNT")
        .unwrap_or_else(|_| "5".to_string())
        .parse::<i64>()
        .unwrap();
    let mut symbols = Vec::<String>::new();
    for i in 0..retry_count {
        match fetch_symbols(exchange, market_type) {
            Ok(list) => {
                symbols = list;
                break;
            }
            Err(err) => {
                std::thread::sleep(Duration::from_secs(60));
                if i == retry_count - 1 {
                    error!("The {}th time, {}", i, err);
                } else {
                    warn!("The {}th time, {}", i, err);
                }
            }
        }
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
        .filter(|symbol| !valid_symbols.contains(symbol))
        .cloned()
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
    loop {
        let mut index = 0_usize;
        let mut success_count = 0_u64;
        let mut back_off_minutes = 0;
        while index < real_symbols.len() {
            let symbol = &real_symbols[index];
            let resp = match msg_type {
                MessageType::L2Snapshot => fetch_l2_snapshot(exchange, market_type, symbol),
                MessageType::L3Snapshot => fetch_l3_snapshot(exchange, market_type, symbol),
                _ => panic!("msg_type must be L2Snapshot or L3Snapshot"),
            };
            // sleep 100ms after each request
            std::thread::sleep(Duration::from_millis(100));
            match resp {
                Ok(msg) => {
                    index += 1;
                    success_count += 1;
                    back_off_minutes = 0;
                    let message = Message::new(exchange.to_string(), market_type, msg_type, msg);
                    (on_msg.lock().unwrap())(message);
                }
                Err(err) => {
                    let current_timestamp = SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64;
                    if err.0.contains("429") || err.0.contains("418") {
                        let next_minute = (current_timestamp as f64 / (1000_f64 * 60_f64)).ceil()
                            * (1000_f64 * 60_f64);
                        let duration =
                            next_minute as u64 - current_timestamp + 60000 * back_off_minutes + 1;
                        warn!(
                            "{} {} {} {} {} {}, error: {}, back off for {} milliseconds",
                            current_timestamp,
                            success_count,
                            back_off_minutes,
                            exchange,
                            market_type,
                            symbol,
                            err,
                            duration
                        );
                        success_count = 0;
                        back_off_minutes += 1;
                        std::thread::sleep(Duration::from_millis(duration));
                    } else if err.0.contains("403") {
                        // 403 is very serious, we should stop crawling
                        success_count = 0;
                        back_off_minutes = if back_off_minutes == 0 {
                            1
                        } else {
                            back_off_minutes * 2
                        };
                        error!(
                            "{} {} {} {} {} {}, error: {}, back off for {} minutes",
                            current_timestamp,
                            success_count,
                            back_off_minutes,
                            exchange,
                            market_type,
                            symbol,
                            err,
                            back_off_minutes
                        );
                        std::thread::sleep(Duration::from_secs(back_off_minutes * 60));
                    } else {
                        error!("{} {} {}, error: {}", exchange, market_type, symbol, err);
                    }
                }
            }
        }
        std::thread::sleep(Duration::from_secs(2)); // sleep 2 seconds after each round
        if let Some(seconds) = duration {
            if now.elapsed() > Duration::from_secs(seconds) {
                break;
            }
        }
    }
}

macro_rules! gen_crawl_event {
    ($func_name:ident, $struct_name:ident, $msg_type:expr, $crawl_func:ident) => {
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

            let real_symbols = if is_empty {
                fetch_symbols_retry(EXCHANGE_NAME, market_type)
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

            if real_symbols.len() <= MAX_SUBSCRIPTIONS_PER_CONNECTION {
                let should_stop = Arc::new(AtomicBool::new(false));
                let ws_client = Arc::new($struct_name::new(on_msg_ext, None));

                if symbols.is_none() {
                    let should_stop2 = should_stop.clone();
                    let ws_client2 = ws_client.clone();

                    let mut subscribed_symbols = real_symbols.clone();
                    std::thread::spawn(move || {
                        while !should_stop2.load(Ordering::Acquire) {
                            let latest_symbols = fetch_symbols_retry(EXCHANGE_NAME, market_type);
                            let mut new_symbols: Vec<String> = latest_symbols
                                .iter()
                                .filter(|s| !subscribed_symbols.contains(s))
                                .cloned()
                                .collect();

                            if !new_symbols.is_empty() {
                                warn!("Found new symbols: {}", new_symbols.join(", "));
                                ws_client2.$crawl_func(&new_symbols);
                                subscribed_symbols.append(&mut new_symbols);
                            }
                            // update symbols every hour
                            std::thread::sleep(Duration::from_secs(3600));
                        }
                    });
                }

                ws_client.$crawl_func(&real_symbols);
                ws_client.run(duration);
                should_stop.store(true, Ordering::Release);
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

                if symbols.is_none() {
                    let num_threads = Arc::new(AtomicUsize::new(chunks.len()));
                    let last_client = Arc::new($struct_name::new(on_msg_ext.clone(), None));

                    for chunk in chunks.into_iter() {
                        let on_msg_ext_clone = on_msg_ext.clone();
                        let num_threads_clone = num_threads.clone();
                        let last_client_clone = last_client.clone();
                        std::thread::spawn(move || {
                            if chunk.len() < MAX_SUBSCRIPTIONS_PER_CONNECTION {
                                last_client_clone.$crawl_func(&chunk);
                                last_client_clone.run(duration);
                            } else {
                                let ws_client = $struct_name::new(on_msg_ext_clone, None);
                                ws_client.$crawl_func(&chunk);
                                ws_client.run(duration);
                            }

                            num_threads_clone.fetch_sub(1, Ordering::SeqCst);
                        });
                    }

                    let mut subscribed_symbols = real_symbols.clone();
                    while num_threads.load(Ordering::Acquire) > 0 {
                        let latest_symbols = fetch_symbols_retry(EXCHANGE_NAME, market_type);
                        let mut new_symbols: Vec<String> = latest_symbols
                            .iter()
                            .filter(|s| !subscribed_symbols.contains(s))
                            .cloned()
                            .collect();
                        if !new_symbols.is_empty() {
                            warn!("Found new symbols: {}", new_symbols.join(", "));
                            last_client.$crawl_func(&new_symbols);
                            subscribed_symbols.append(&mut new_symbols);
                        }
                        // update symbols every hour
                        std::thread::sleep(Duration::from_secs(duration.unwrap_or(3600)));
                    }
                } else {
                    let mut join_handles: Vec<std::thread::JoinHandle<()>> = Vec::new();

                    for chunk in chunks.into_iter() {
                        let on_msg_ext_clone = on_msg_ext.clone();
                        let handle = std::thread::spawn(move || {
                            let ws_client = $struct_name::new(on_msg_ext_clone, None);
                            ws_client.$crawl_func(&chunk);
                            ws_client.run(duration);
                        });
                        join_handles.push(handle);
                    }
                    for handle in join_handles {
                        handle.join().unwrap();
                    }
                }
            }
            None
        }
    };
}
