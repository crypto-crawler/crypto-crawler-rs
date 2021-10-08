use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant, SystemTime},
};

use crate::utils::{REST_LOCKS, WS_LOCKS};
use crypto_markets::{fetch_symbols, get_market_types, MarketType};
use crypto_rest_client::{fetch_l2_snapshot, fetch_l3_snapshot};
use crypto_ws_client::*;
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
        if !lock_.owns_lock() {
            lock_.lock().unwrap();
        }
        match fetch_symbols(exchange, market_type) {
            Ok(list) => {
                symbols = list;
                if !lock_.owns_lock() {
                    lock_.lock().unwrap();
                }
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
        if lock_.owns_lock() {
            lock_.unlock().unwrap();
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
            if !lock_.owns_lock() {
                lock_.lock().unwrap();
            }
            let resp = match msg_type {
                MessageType::L2Snapshot => fetch_l2_snapshot(exchange, market_type, symbol, None),
                MessageType::L3Snapshot => fetch_l3_snapshot(exchange, market_type, symbol, None),
                _ => panic!("msg_type must be L2Snapshot or L3Snapshot"),
            };
            // Cooldown after each request, and make all other processes wait
            // on the lock to avoid parallel requests, thus avoid 429 error
            std::thread::sleep(cooldown_time);
            if lock_.owns_lock() {
                lock_.unlock().unwrap();
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

pub(crate) fn subscribe_with_lock_new(
    exchange: &str,
    market_type: MarketType,
    msg_type: MessageType,
    symbols: &[String],
    ws_client: Arc<dyn WSClient>,
) {
    let lock = WS_LOCKS
        .get(exchange)
        .unwrap()
        .get(&market_type)
        .unwrap()
        .clone();
    let mut lock = lock.lock().unwrap();
    let interval = get_send_interval_ms(exchange, market_type);
    if interval.is_some() && !lock.owns_lock() {
        lock.lock().unwrap();
    }
    match msg_type {
        MessageType::BBO => ws_client.subscribe_bbo(symbols),
        MessageType::Trade => ws_client.subscribe_trade(symbols),
        MessageType::L2Event => ws_client.subscribe_orderbook(symbols),
        MessageType::L3Event => ws_client.subscribe_l3_orderbook(symbols),
        MessageType::L2TopK => ws_client.subscribe_orderbook_topk(symbols),
        MessageType::Ticker => ws_client.subscribe_ticker(symbols),
        _ => panic!(
            "{} {} does NOT have {} websocket channel",
            exchange, market_type, msg_type
        ),
    };
    if let Some(interval) = interval {
        std::thread::sleep(Duration::from_millis(interval));
        if lock.owns_lock() {
            lock.unlock().unwrap();
        }
    }
}

pub(crate) fn subscribe_candlestick_with_lock(
    exchange: &str,
    market_type: MarketType,
    symbol_interval_list: &[(String, usize)],
    ws_client: Arc<dyn WSClient>,
) {
    let lock = WS_LOCKS
        .get(exchange)
        .unwrap()
        .get(&market_type)
        .unwrap()
        .clone();
    let mut lock = lock.lock().unwrap();
    let interval = get_send_interval_ms(exchange, market_type);
    if interval.is_some() && !lock.owns_lock() {
        lock.lock().unwrap();
    }
    ws_client.subscribe_candlestick(symbol_interval_list);
    if let Some(interval) = interval {
        std::thread::sleep(Duration::from_millis(interval));
        if lock.owns_lock() {
            lock.unlock().unwrap();
        }
    }
}

fn get_num_subscriptions_per_connection(exchange: &str) -> usize {
    match exchange {
        "binance" => 200, // https://binance-docs.github.io/apidocs/futures/en/#websocket-market-streams
        // A single connection can listen to a maximum of 200 streams
        "bitfinex" => 30, // https://docs.bitfinex.com/docs/ws-general#subscribe-to-channels
        // All websocket connections have a limit of 30 subscriptions to public market data feed channels
        "kucoin" => 300, // https://docs.kucoin.cc/#request-rate-limit
        // Subscription limit for each connection: 300 topics
        "okex" => 256,   // https://www.okex.com/docs/zh/#question-public
        _ => usize::MAX, // usize::MAX means unlimited
    }
}

fn create_ws_client(
    exchange: &str,
    market_type: MarketType,
    msg_type: MessageType,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
) -> Arc<dyn WSClient<'static> + Send + Sync> {
    let lock = WS_LOCKS
        .get(exchange)
        .unwrap()
        .get(&market_type)
        .unwrap()
        .clone();
    let mut lock = lock.lock().unwrap();
    let interval = get_connection_interval_ms(exchange, market_type);
    if interval.is_some() && !lock.owns_lock() {
        lock.lock().unwrap();
    }
    let exchange_clone = exchange.to_string();
    let on_msg_ext = Arc::new(Mutex::new(move |msg: String| {
        let message = Message::new(exchange_clone.clone(), market_type, msg_type, msg);
        (on_msg.lock().unwrap())(message);
    }));

    let ws_client: Arc<dyn WSClient<'static> + Send + Sync> = match exchange {
        "binance" => match market_type {
            MarketType::Spot => Arc::new(BinanceSpotWSClient::new(on_msg_ext, None)),
            MarketType::InverseFuture | MarketType::InverseSwap => {
                Arc::new(BinanceInverseWSClient::new(on_msg_ext, None))
            }
            MarketType::LinearFuture | MarketType::LinearSwap => {
                Arc::new(BinanceLinearWSClient::new(on_msg_ext, None))
            }
            MarketType::EuropeanOption => Arc::new(BinanceOptionWSClient::new(on_msg_ext, None)),
            _ => panic!("Binance does NOT have the {} market type", market_type),
        },
        "bitfinex" => Arc::new(BitfinexWSClient::new(on_msg_ext, None)),
        "bitget" => match market_type {
            MarketType::InverseSwap | MarketType::LinearSwap => {
                Arc::new(BitgetSwapWSClient::new(on_msg_ext, None))
            }
            _ => panic!("Bitget does NOT have the {} market type", market_type),
        },
        "bithumb" => Arc::new(BithumbWSClient::new(on_msg_ext, None)),
        "bitmex" => Arc::new(BitmexWSClient::new(on_msg_ext, None)),
        "bitstamp" => Arc::new(BitstampWSClient::new(on_msg_ext, None)),
        "bitz" => match market_type {
            MarketType::Spot => Arc::new(BitzSpotWSClient::new(on_msg_ext, None)),
            _ => panic!("Bitz does NOT have the {} market type", market_type),
        },
        "bybit" => match market_type {
            MarketType::InverseFuture => {
                Arc::new(BybitInverseFutureWSClient::new(on_msg_ext, None))
            }
            MarketType::InverseSwap => Arc::new(BybitInverseSwapWSClient::new(on_msg_ext, None)),
            MarketType::LinearSwap => Arc::new(BybitLinearSwapWSClient::new(on_msg_ext, None)),
            _ => panic!("Bybit does NOT have the {} market type", market_type),
        },
        "coinbase_pro" => Arc::new(CoinbaseProWSClient::new(on_msg_ext, None)),
        "deribit" => Arc::new(DeribitWSClient::new(on_msg_ext, None)),
        "ftx" => Arc::new(FtxWSClient::new(on_msg_ext, None)),
        "gate" => match market_type {
            MarketType::Spot => Arc::new(GateSpotWSClient::new(on_msg_ext, None)),
            MarketType::InverseSwap => Arc::new(GateInverseSwapWSClient::new(on_msg_ext, None)),
            MarketType::LinearSwap => Arc::new(GateLinearSwapWSClient::new(on_msg_ext, None)),
            MarketType::LinearFuture => Arc::new(GateLinearFutureWSClient::new(on_msg_ext, None)),
            _ => panic!("Gate does NOT have the {} market type", market_type),
        },
        "huobi" => match market_type {
            MarketType::Spot => Arc::new(HuobiSpotWSClient::new(on_msg_ext, None)),
            MarketType::InverseFuture => Arc::new(HuobiFutureWSClient::new(on_msg_ext, None)),
            MarketType::LinearSwap => Arc::new(HuobiLinearSwapWSClient::new(on_msg_ext, None)),
            MarketType::InverseSwap => Arc::new(HuobiInverseSwapWSClient::new(on_msg_ext, None)),
            MarketType::EuropeanOption => Arc::new(HuobiOptionWSClient::new(on_msg_ext, None)),
            _ => panic!("Huobi does NOT have the {} market type", market_type),
        },
        "kraken" => Arc::new(KrakenWSClient::new(on_msg_ext, None)),
        "kucoin" => match market_type {
            MarketType::Spot => Arc::new(KuCoinSpotWSClient::new(on_msg_ext, None)),
            MarketType::InverseSwap | MarketType::LinearSwap | MarketType::InverseFuture => {
                Arc::new(KuCoinSwapWSClient::new(on_msg_ext, None))
            }
            _ => panic!("KuCoin does NOT have the {} market type", market_type),
        },
        "mxc" => match market_type {
            MarketType::Spot => Arc::new(MxcSpotWSClient::new(on_msg_ext, None)),
            MarketType::LinearSwap | MarketType::InverseSwap => {
                Arc::new(MxcSwapWSClient::new(on_msg_ext, None))
            }
            _ => panic!("MXC does NOT have the {} market type", market_type),
        },
        "okex" => Arc::new(OkexWSClient::new(on_msg_ext, None)),
        "zbg" => match market_type {
            MarketType::Spot => Arc::new(ZbgSpotWSClient::new(on_msg_ext, None)),
            MarketType::InverseSwap | MarketType::LinearSwap => {
                Arc::new(ZbgSwapWSClient::new(on_msg_ext, None))
            }
            _ => panic!("ZBG does NOT have the {} market type", market_type),
        },
        _ => panic!("Unknown exchange {}", exchange),
    };
    if let Some(interval) = interval {
        std::thread::sleep(Duration::from_millis(interval));
        if lock.owns_lock() {
            lock.unlock().unwrap();
        }
    }
    ws_client
}

pub(crate) fn crawl_event(
    exchange: &str,
    msg_type: MessageType,
    market_type: MarketType,
    symbols: Option<&[String]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    let num_topics_per_connection = get_num_subscriptions_per_connection(exchange);
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
    let automatic_symbol_discovery = is_empty && duration.is_none();

    let real_symbols = if is_empty {
        if exchange == "binance" {
            fetch_symbols_retry(exchange, market_type)
                .into_iter()
                .map(|s| s.to_lowercase())
                .collect()
        } else {
            fetch_symbols_retry(exchange, market_type)
        }
    } else {
        symbols.unwrap().to_vec()
    };
    if real_symbols.is_empty() {
        panic!("real_symbols is empty");
    }

    let exchange_clone = exchange.to_string();
    let on_msg_clone = on_msg.clone();
    let mut subscribed_symbols = real_symbols.clone();
    let create_symbol_discovery_thread = move || -> std::thread::JoinHandle<()> {
        std::thread::spawn(move || {
            let ws_client =
                create_ws_client(exchange_clone.as_str(), market_type, msg_type, on_msg_clone);
            let ws_client_clone = ws_client.clone();
            let should_stop = Arc::new(AtomicBool::new(false));
            let should_stop_clone = should_stop.clone();
            std::thread::spawn(move || {
                let exchange: &str = exchange_clone.as_str();
                let mut total_new_symbols = 0;
                while !should_stop_clone.load(Ordering::Acquire) {
                    // update symbols every hour
                    std::thread::sleep(Duration::from_secs(3600));
                    let latest_symbols = if exchange == "binance" {
                        fetch_symbols_retry(exchange, market_type)
                            .into_iter()
                            .map(|s| s.to_lowercase())
                            .collect()
                    } else {
                        fetch_symbols_retry(exchange, market_type)
                    };
                    let mut new_symbols: Vec<String> = latest_symbols
                        .iter()
                        .filter(|s| !subscribed_symbols.contains(s))
                        .cloned()
                        .collect();

                    if !new_symbols.is_empty() {
                        warn!("Found new symbols: {}", new_symbols.join(", "));
                        subscribe_with_lock_new(
                            exchange,
                            market_type,
                            msg_type,
                            &new_symbols,
                            ws_client_clone.clone(),
                        );
                        subscribed_symbols.append(&mut new_symbols);
                    }
                    total_new_symbols += new_symbols.len();
                    if total_new_symbols > num_topics_per_connection {
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

    if real_symbols.len() <= num_topics_per_connection {
        let ws_client = create_ws_client(exchange, market_type, msg_type, on_msg);
        subscribe_with_lock_new(
            exchange,
            market_type,
            msg_type,
            &real_symbols,
            ws_client.clone(),
        );
        ws_client.run(duration);
        ws_client.close();
    } else {
        // split to chunks
        let mut chunks: Vec<Vec<String>> = Vec::new();
        for i in (0..real_symbols.len()).step_by(num_topics_per_connection) {
            let chunk = (&real_symbols
                [i..(std::cmp::min(i + num_topics_per_connection, real_symbols.len()))])
                .to_vec();
            chunks.push(chunk);
        }
        assert!(chunks.len() > 1);

        let mut join_handles: Vec<std::thread::JoinHandle<()>> = Vec::new();
        for chunk in chunks.into_iter() {
            let exchange_clone = exchange.to_string();
            let on_msg_clone = on_msg.clone();
            let handle = std::thread::spawn(move || {
                let exchange: &str = exchange_clone.as_str();
                let ws_client = create_ws_client(exchange, market_type, msg_type, on_msg_clone);
                subscribe_with_lock_new(exchange, market_type, msg_type, &chunk, ws_client.clone());
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
        "okex" => Some(1000), //  Connection limit：1 times/s, https://www.okex.com/docs/en/#spot_ws-limit
        _ => Some(20),
    }
}

pub(crate) fn get_send_interval_ms(exchange: &str, _market_type: MarketType) -> Option<u64> {
    match exchange {
        "binance" => Some(100), // WebSocket connections have a limit of 10 incoming messages per second
        "kucoin" => Some(100),  //  Message limit sent to the server: 100 per 10 seconds
        // "okex" => Some(15000), // 240 times/hour, https://www.okex.com/docs/en/#spot_ws-limit
        _ => None,
    }
}

pub(crate) fn crawl_candlestick_ext(
    exchange: &str,
    market_type: MarketType,
    symbol_interval_list: Option<&[(String, usize)]>,
    on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
    duration: Option<u64>,
) -> Option<std::thread::JoinHandle<()>> {
    let num_topics_per_connection = get_num_subscriptions_per_connection(exchange);
    let is_empty = match symbol_interval_list {
        Some(list) => {
            if list.is_empty() {
                true
            } else {
                let symbols: Vec<String> = list.iter().map(|t| t.0.clone()).collect();
                check_args(exchange, market_type, symbols.as_slice());
                false
            }
        }
        None => true,
    };
    let automatic_symbol_discovery = is_empty && duration.is_none();

    let symbol_interval_list: Vec<(String, usize)> = if is_empty {
        let symbols = if exchange == "binance" {
            fetch_symbols_retry(exchange, market_type)
                .into_iter()
                .map(|s| s.to_lowercase())
                .collect()
        } else {
            fetch_symbols_retry(exchange, market_type)
        };
        let intervals = get_candlestick_intervals(exchange, market_type);
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
        symbol_interval_list.unwrap().to_vec()
    };
    if symbol_interval_list.is_empty() {
        panic!("symbol_interval_list is empty");
    }
    let real_symbols: Vec<String> = symbol_interval_list.iter().map(|t| t.0.clone()).collect();
    let real_intervals: Vec<usize> = symbol_interval_list.iter().map(|t| t.1).collect();

    let thread = if automatic_symbol_discovery {
        let exchange_clone = exchange.to_string();
        let on_msg_clone = on_msg.clone();
        let mut subscribed_symbols = real_symbols;
        let thread = std::thread::spawn(move || {
            let ws_client = create_ws_client(
                exchange_clone.as_str(),
                market_type,
                MessageType::Candlestick,
                on_msg_clone,
            );
            let ws_client_clone = ws_client.clone();
            let should_stop = Arc::new(AtomicBool::new(false));
            let should_stop_clone = should_stop.clone();
            std::thread::spawn(move || {
                let exchange: &str = exchange_clone.as_str();
                let mut total_new_symbols = 0;
                while !should_stop_clone.load(Ordering::Acquire) {
                    // update symbols every hour
                    std::thread::sleep(Duration::from_secs(3600));
                    let latest_symbols = if exchange == "binance" {
                        fetch_symbols_retry(exchange, market_type)
                            .into_iter()
                            .map(|s| s.to_lowercase())
                            .collect()
                    } else {
                        fetch_symbols_retry(exchange, market_type)
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
                                real_intervals
                                    .clone()
                                    .into_iter()
                                    .map(move |interval| (symbol.clone(), interval))
                            })
                            .collect::<Vec<(String, usize)>>();
                        subscribe_candlestick_with_lock(
                            exchange,
                            market_type,
                            new_symbol_interval_list.as_slice(),
                            ws_client_clone.clone(),
                        );
                        subscribed_symbols.append(&mut new_symbols);
                    }
                    total_new_symbols += new_symbols.len();
                    if total_new_symbols > num_topics_per_connection {
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

    if symbol_interval_list.len() <= num_topics_per_connection {
        let ws_client = create_ws_client(exchange, market_type, MessageType::Candlestick, on_msg);
        subscribe_candlestick_with_lock(
            exchange,
            market_type,
            symbol_interval_list.as_slice(),
            ws_client.clone(),
        );
        ws_client.run(duration);
        ws_client.close();
    } else {
        // split to chunks
        let mut chunks: Vec<Vec<(String, usize)>> = Vec::new();
        for i in (0..symbol_interval_list.len()).step_by(num_topics_per_connection) {
            let chunk: Vec<(String, usize)> = (&symbol_interval_list
                [i..(std::cmp::min(i + num_topics_per_connection, symbol_interval_list.len()))])
                .to_vec();
            chunks.push(chunk);
        }
        assert!(chunks.len() > 1);

        let mut join_handles: Vec<std::thread::JoinHandle<()>> = Vec::new();
        for chunk in chunks.into_iter() {
            let exchange_clone = exchange.to_string();
            let on_msg_clone = on_msg.clone();
            let handle = std::thread::spawn(move || {
                let exchange: &str = exchange_clone.as_str();
                let ws_client = create_ws_client(
                    exchange,
                    market_type,
                    MessageType::Candlestick,
                    on_msg_clone,
                );
                subscribe_candlestick_with_lock(
                    exchange,
                    market_type,
                    chunk.as_slice(),
                    ws_client.clone(),
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
