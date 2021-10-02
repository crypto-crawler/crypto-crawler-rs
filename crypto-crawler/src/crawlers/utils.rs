use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant, SystemTime},
};

use crypto_markets::{fetch_symbols, get_market_types, MarketType};
use crypto_rest_client::{fetch_l2_snapshot, fetch_l3_snapshot};
use fslock::LockFile;
use log::*;

use crate::{get_hot_spot_symbols, utils::cmc_rank::sort_by_cmc_rank, Message, MessageType};

fn get_lock_file(exchange: &str, market_type: MarketType) -> Option<LockFile> {
    let filename = if exchange == "bitmex" {
        Some("bitmex.lock")
    } else if exchange == "binance" {
        if market_type == MarketType::InverseSwap || market_type == MarketType::InverseFuture {
            Some("binance_inverse.lock")
        } else if market_type == MarketType::LinearFuture || market_type == MarketType::LinearSwap {
            Some("binance_linear.lock")
        } else {
            None
        }
    } else {
        None
    };
    if let Some(filename) = filename {
        let mut dir = if std::env::var("DATA_DIR").is_ok() {
            let dir =
                std::path::Path::new(std::env::var("DATA_DIR").unwrap().as_str()).join("lock");
            std::fs::create_dir_all(&dir).unwrap();
            dir
        } else {
            std::env::temp_dir()
        };
        dir.push(filename);
        let file = LockFile::open(dir.as_path()).unwrap();
        Some(file)
    } else {
        None
    }
}

pub fn fetch_symbols_retry(exchange: &str, market_type: MarketType) -> Vec<String> {
    let retry_count = std::env::var("REST_RETRY_COUNT")
        .unwrap_or_else(|_| "5".to_string())
        .parse::<i64>()
        .unwrap();
    let cooldown_time = get_cooldown_time_per_request(exchange, market_type);
    let mut lock = get_lock_file(exchange, market_type);
    let mut symbols = Vec::<String>::new();
    let mut backoff_factor = 1;
    for i in 0..retry_count {
        if let Some(ref mut lock) = lock {
            lock.lock().unwrap();
        }
        match fetch_symbols(exchange, market_type) {
            Ok(list) => {
                symbols = list;
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
        if let Some(ref mut lock) = lock {
            // Cooldown after each request, and make all other processes wait
            // on the lock to avoid parallel requests, thus avoid 429 error
            std::thread::sleep(cooldown_time * backoff_factor);
            lock.unlock().unwrap();
        } else {
            // Cooldown after each request
            std::thread::sleep(cooldown_time * backoff_factor);
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
            MarketType::Spot => 200, // addtional 10ms to avoid 429
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

    let mut lock = get_lock_file(exchange, market_type);
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
            if let Some(ref mut lock) = lock {
                lock.lock().unwrap();
            }
            let resp = match msg_type {
                MessageType::L2Snapshot => fetch_l2_snapshot(exchange, market_type, symbol, None),
                MessageType::L3Snapshot => fetch_l3_snapshot(exchange, market_type, symbol, None),
                _ => panic!("msg_type must be L2Snapshot or L3Snapshot"),
            };
            if let Some(ref mut lock) = lock {
                // Cooldown after each request, and make all other processes wait
                // on the lock to avoid parallel requests, thus avoid 429 error
                std::thread::sleep(cooldown_time);
                lock.unlock().unwrap();
            } else {
                // Cooldown after each request
                std::thread::sleep(cooldown_time);
            }
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
                    if err.0.contains("429") {
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
                    let should_stop_clone = should_stop.clone();
                    let ws_client2 = ws_client.clone();

                    let mut subscribed_symbols = real_symbols.clone();
                    std::thread::spawn(move || {
                        while !should_stop_clone.load(Ordering::Acquire) {
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
                ws_client.close();
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
                                last_client_clone.close();
                            } else {
                                let ws_client = $struct_name::new(on_msg_ext_clone, None);
                                ws_client.$crawl_func(&chunk);
                                ws_client.run(duration);
                                ws_client.close();
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
                            ws_client.close();
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
