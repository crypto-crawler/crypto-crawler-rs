use std::{
    sync::{mpsc::Sender, Arc},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::utils::{REST_LOCKS, WS_LOCKS};
use crypto_market_type::{get_market_types, MarketType};
use crypto_markets::fetch_symbols;
use crypto_rest_client::{fetch_l2_snapshot, fetch_l3_snapshot, fetch_open_interest};
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
                if lock_.owns_lock() {
                    lock_.unlock().unwrap();
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
        "dydx" => 100,      // 100 requests per 10 seconds
        "gate" => 4,        // 300 read operations per IP per second
        "huobi" => 2,       // 800 times/second for one IP
        "kucoin" => match market_type {
            MarketType::Spot => 300, // 3x to avoid 429
            _ => 100,                // 30 times/3s
        },
        "mexc" => 100, // 20 times per 2 seconds
        "okx" => 100,  // 20 requests per 2 seconds
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
    tx: Sender<Message>,
) {
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
    'outer: loop {
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
        // retry 5 times at most
        while index < real_symbols.len() && backoff_factor < 6 {
            let symbol = real_symbols[index].as_str();
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
                    let message = Message::new_with_symbol(
                        exchange.to_string(),
                        market_type,
                        msg_type,
                        symbol.to_string(),
                        msg,
                    );
                    if tx.send(message).is_err() {
                        // break the loop if there is no receiver
                        break 'outer;
                    }
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
                    backoff_factor += 1;
                }
            }
        }
        std::thread::sleep(cooldown_time * 2); // if real_symbols is empty, CPU will be 100% without this line
    }
}

/// Crawl open interests of all trading symbols.
pub(crate) fn crawl_open_interest(exchange: &str, market_type: MarketType, tx: Sender<Message>) {
    let cooldown_time = get_cooldown_time_per_request(exchange, market_type);

    let lock = REST_LOCKS
        .get(exchange)
        .unwrap()
        .get(&market_type)
        .unwrap()
        .clone();
    'outer: loop {
        match exchange {
            "bitz" | "deribit" | "dydx" | "ftx" | "huobi" | "kucoin" | "okx" => {
                let mut lock_ = lock.lock().unwrap();
                if !lock_.owns_lock() {
                    lock_.lock().unwrap();
                }
                let resp = fetch_open_interest(exchange, market_type, None);
                if let Ok(json) = resp {
                    if exchange == "deribit" {
                        // A RESTful response of deribit open_interest contains four lines
                        for x in json.trim().split('\n') {
                            let message = Message::new(
                                exchange.to_string(),
                                market_type,
                                MessageType::OpenInterest,
                                x.to_string(),
                            );
                            if tx.send(message).is_err() {
                                break; // break the loop if there is no receiver
                            }
                        }
                    } else {
                        let message = Message::new(
                            exchange.to_string(),
                            market_type,
                            MessageType::OpenInterest,
                            json,
                        );
                        if tx.send(message).is_err() {
                            break; // break the loop if there is no receiver
                        }
                    }
                }
                // Cooldown after each request, and make all other processes wait
                // on the lock to avoid parallel requests, thus avoid 429 error
                std::thread::sleep(cooldown_time);
                if lock_.owns_lock() {
                    lock_.unlock().unwrap();
                }
            }
            "binance" | "bitget" | "bybit" | "gate" | "zbg" => {
                let real_symbols = fetch_symbols_retry(exchange, market_type);

                let mut index = 0_usize;
                let mut success_count = 0_u64;
                let mut backoff_factor = 1;
                // retry 5 times at most
                while index < real_symbols.len() && backoff_factor < 6 {
                    let symbol = real_symbols[index].as_str();
                    let mut lock_ = lock.lock().unwrap();
                    if !lock_.owns_lock() {
                        lock_.lock().unwrap();
                    }
                    let resp = fetch_open_interest(exchange, market_type, Some(symbol));
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
                            let message = Message::new_with_symbol(
                                exchange.to_string(),
                                market_type,
                                MessageType::OpenInterest,
                                symbol.to_string(),
                                msg,
                            );
                            if tx.send(message).is_err() {
                                // break the loop if there is no receiver
                                break 'outer;
                            }
                        }
                        Err(err) => {
                            let current_timestamp = SystemTime::now()
                                .duration_since(SystemTime::UNIX_EPOCH)
                                .unwrap()
                                .as_millis()
                                as u64;
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
                            backoff_factor += 1;
                        }
                    }
                }
            }
            _ => panic!("{} does NOT have open interest RESTful API", exchange),
        }
        std::thread::sleep(cooldown_time * 2); // if real_symbols is empty, CPU will be 100% without this line
    }
}

async fn subscribe_with_lock(
    exchange: String,
    market_type: MarketType,
    msg_type: MessageType,
    symbols: Vec<String>,
    ws_client: Arc<dyn WSClient + Send + Sync>,
) {
    match msg_type {
        MessageType::BBO => ws_client.subscribe_bbo(&symbols).await,
        MessageType::Trade => ws_client.subscribe_trade(&symbols).await,
        MessageType::L2Event => ws_client.subscribe_orderbook(&symbols).await,
        MessageType::L3Event => ws_client.subscribe_l3_orderbook(&symbols).await,
        MessageType::L2TopK => ws_client.subscribe_orderbook_topk(&symbols).await,
        MessageType::Ticker => ws_client.subscribe_ticker(&symbols).await,
        _ => panic!(
            "{} {} does NOT have {} websocket channel",
            exchange, market_type, msg_type
        ),
    };
}

fn get_connection_interval_ms(exchange: &str, _market_type: MarketType) -> Option<u64> {
    match exchange {
        "bitfinex" => Some(3000), // you cannot open more than 20 connections per minute, see https://docs.bitfinex.com/docs/requirements-and-limitations#websocket-rate-limits
        // "bitmex" => Some(9000), // 40 per hour
        "bitz" => Some(100), // `cat crawler-trade-bitz-spot-error-12.log` has many "429 Too Many Requests"
        "kucoin" => Some(2000), //  Connection Limit: 30 per minute, see https://docs.kucoin.com/#connection-times
        "okx" => Some(1000), // Connection limit: 1 time per second, https://www.okx.com/docs-v5/en/#websocket-api-connect
        _ => None,
    }
}

fn get_num_subscriptions_per_connection(exchange: &str, market_type: MarketType) -> usize {
    match exchange {
        // A single connection can listen to a maximum of 200 streams
        "binance" => {
            if market_type == MarketType::Spot {
                // https://binance-docs.github.io/apidocs/spot/en/#websocket-limits
                1024
            } else {
                // https://binance-docs.github.io/apidocs/futures/en/#websocket-market-streams
                // https://binance-docs.github.io/apidocs/delivery/en/#websocket-market-streams
                200
            }
        } // https://binance-docs.github.io/apidocs/futures/en/#websocket-market-streams
        // All websocket connections have a limit of 30 subscriptions to public market data feed channels
        "bitfinex" => 30, // https://docs.bitfinex.com/docs/ws-general#subscribe-to-channels
        "kucoin" => 300, // Subscription limit for each connection: 300 topics, see https://docs.kucoin.com/#topic-subscription-limit
        "okx" => 256,    // okx spot l2_event throws many ResetWithoutClosingHandshake errors
        _ => usize::MAX, // usize::MAX means unlimited
    }
}

async fn create_ws_client_internal(
    exchange: &str,
    market_type: MarketType,
    tx: Sender<String>,
) -> Arc<dyn WSClient + Send + Sync> {
    match exchange {
        "binance" => match market_type {
            MarketType::Spot => Arc::new(BinanceSpotWSClient::new(tx, None).await),
            MarketType::InverseFuture | MarketType::InverseSwap => {
                Arc::new(BinanceInverseWSClient::new(tx, None).await)
            }
            MarketType::LinearFuture | MarketType::LinearSwap => {
                Arc::new(BinanceLinearWSClient::new(tx, None).await)
            }
            MarketType::EuropeanOption => Arc::new(BinanceOptionWSClient::new(tx, None).await),
            _ => panic!("Binance does NOT have the {} market type", market_type),
        },
        "bitfinex" => Arc::new(BitfinexWSClient::new(tx, None).await),
        "bitget" => match market_type {
            MarketType::Spot => Arc::new(BitgetSpotWSClient::new(tx, None).await),
            MarketType::InverseSwap | MarketType::LinearSwap => {
                Arc::new(BitgetSwapWSClient::new(tx, None).await)
            }
            _ => panic!("Bitget does NOT have the {} market type", market_type),
        },
        "bithumb" => Arc::new(BithumbWSClient::new(tx, None).await),
        "bitmex" => Arc::new(BitmexWSClient::new(tx, None).await),
        "bitstamp" => Arc::new(BitstampWSClient::new(tx, None).await),
        "bitz" => match market_type {
            MarketType::Spot => Arc::new(BitzSpotWSClient::new(tx, None).await),
            _ => panic!("Bitz does NOT have the {} market type", market_type),
        },
        "bybit" => match market_type {
            MarketType::InverseFuture | MarketType::InverseSwap => {
                Arc::new(BybitInverseWSClient::new(tx, None).await)
            }
            MarketType::LinearSwap => Arc::new(BybitLinearSwapWSClient::new(tx, None).await),
            _ => panic!("Bybit does NOT have the {} market type", market_type),
        },
        "coinbase_pro" => Arc::new(CoinbaseProWSClient::new(tx, None).await),
        "deribit" => Arc::new(DeribitWSClient::new(tx, None).await),
        "dydx" => match market_type {
            MarketType::LinearSwap => Arc::new(DydxSwapWSClient::new(tx, None).await),
            _ => panic!("dYdX does NOT have the {} market type", market_type),
        },
        "ftx" => Arc::new(FtxWSClient::new(tx, None).await),
        "gate" => match market_type {
            MarketType::Spot => Arc::new(GateSpotWSClient::new(tx, None).await),
            MarketType::InverseSwap => Arc::new(GateInverseSwapWSClient::new(tx, None).await),
            MarketType::LinearSwap => Arc::new(GateLinearSwapWSClient::new(tx, None).await),
            MarketType::InverseFuture => Arc::new(GateInverseFutureWSClient::new(tx, None).await),
            MarketType::LinearFuture => Arc::new(GateLinearFutureWSClient::new(tx, None).await),
            _ => panic!("Gate does NOT have the {} market type", market_type),
        },
        "huobi" => match market_type {
            MarketType::Spot => Arc::new(HuobiSpotWSClient::new(tx, None).await),
            MarketType::InverseFuture => Arc::new(HuobiFutureWSClient::new(tx, None).await),
            MarketType::LinearSwap => Arc::new(HuobiLinearSwapWSClient::new(tx, None).await),
            MarketType::InverseSwap => Arc::new(HuobiInverseSwapWSClient::new(tx, None).await),
            MarketType::EuropeanOption => Arc::new(HuobiOptionWSClient::new(tx, None).await),
            _ => panic!("Huobi does NOT have the {} market type", market_type),
        },
        "kraken" => match market_type {
            MarketType::Spot => Arc::new(KrakenSpotWSClient::new(tx, None).await),
            MarketType::InverseFuture | MarketType::InverseSwap => {
                Arc::new(KrakenFuturesWSClient::new(tx, None).await)
            }
            _ => panic!("Kraken does NOT have the {} market type", market_type),
        },
        "kucoin" => match market_type {
            MarketType::Spot => Arc::new(KuCoinSpotWSClient::new(tx, None).await),
            MarketType::InverseSwap | MarketType::LinearSwap | MarketType::InverseFuture => {
                Arc::new(KuCoinSwapWSClient::new(tx, None).await)
            }
            _ => panic!("KuCoin does NOT have the {} market type", market_type),
        },
        "mexc" => match market_type {
            MarketType::Spot => Arc::new(MexcSpotWSClient::new(tx, None).await),
            MarketType::LinearSwap | MarketType::InverseSwap => {
                Arc::new(MexcSwapWSClient::new(tx, None).await)
            }
            _ => panic!("MEXC does NOT have the {} market type", market_type),
        },
        "okx" => Arc::new(OkxWSClient::new(tx, None).await),
        "zb" => match market_type {
            MarketType::Spot => Arc::new(ZbSpotWSClient::new(tx, None).await),
            MarketType::LinearSwap => Arc::new(ZbSwapWSClient::new(tx, None).await),
            _ => panic!("ZB does NOT have the {} market type", market_type),
        },
        "zbg" => match market_type {
            MarketType::Spot => Arc::new(ZbgSpotWSClient::new(tx, None).await),
            MarketType::InverseSwap | MarketType::LinearSwap => {
                Arc::new(ZbgSwapWSClient::new(tx, None).await)
            }
            _ => panic!("ZBG does NOT have the {} market type", market_type),
        },
        _ => panic!("Unknown exchange {}", exchange),
    }
}

async fn create_ws_client(
    exchange: &str,
    market_type: MarketType,
    msg_type: MessageType,
    tx: Sender<Message>,
) -> Arc<dyn WSClient + Send + Sync> {
    let tx = create_conversion_thread(exchange.to_string(), msg_type, market_type, tx);
    if let Some(interval) = get_connection_interval_ms(exchange, market_type) {
        let lock = WS_LOCKS
            .get(exchange)
            .unwrap()
            .get(&market_type)
            .unwrap()
            .clone();
        let mut lock = lock.lock().await;
        let mut i = 0;
        while !lock.owns_lock() {
            i += 1;
            debug!(
                "{} {} {} try_lock_with_pid() the {}th time",
                exchange, market_type, msg_type, i
            );
            if lock.try_lock_with_pid().is_ok() {
                break;
            } else {
                tokio::time::sleep(std::time::Duration::from_millis(
                    rand::random::<u64>() % 90 + 11,
                ))
                .await; // give chances to other tasks
            }
        }
        let ws_client = create_ws_client_internal(exchange, market_type, tx).await;
        tokio::time::sleep(Duration::from_millis(interval)).await;
        if lock.owns_lock() {
            lock.unlock().unwrap();
        }
        ws_client
    } else {
        create_ws_client_internal(exchange, market_type, tx).await
    }
}

pub(crate) async fn create_ws_client_symbol(
    exchange: &str,
    market_type: MarketType,
    tx: Sender<String>,
) -> Arc<dyn WSClient + Send + Sync> {
    let tx = create_parser_thread(exchange.to_string(), market_type, tx);
    create_ws_client_internal(exchange, market_type, tx).await
}

#[derive(Clone)]
struct EmptyStruct {} // for stop channel

fn create_symbol_discovery_thread(
    exchange: String,
    market_type: MarketType,
    subscribed_symbols: Vec<String>,
    mut stop_ch_rx: tokio::sync::broadcast::Receiver<EmptyStruct>,
    tx: tokio::sync::mpsc::Sender<Vec<String>>, // send out new symbols
) -> tokio::task::JoinHandle<()> {
    let num_topics_per_connection = get_num_subscriptions_per_connection(&exchange, market_type);
    let mut subscribed_symbols = subscribed_symbols;
    let mut num_subscribed_of_last_client = subscribed_symbols.len() % num_topics_per_connection;
    let mut hourly = tokio::time::interval(Duration::from_secs(3600));
    tokio::task::spawn(async move {
        loop {
            tokio::select! {
                _ = stop_ch_rx.recv() => {
                    break;
                }
                _ = hourly.tick() => {
                    let exchange_clone = exchange.to_string();
                    let latest_symbols = tokio::task::block_in_place(move || {
                        fetch_symbols_retry(&exchange_clone, market_type)
                    });

                    let mut new_symbols: Vec<String> = latest_symbols
                        .iter()
                        .filter(|s| !subscribed_symbols.contains(s))
                        .cloned()
                        .collect();

                    if !new_symbols.is_empty() {
                        warn!("Found new symbols: {}", new_symbols.join(", "));
                        if tx.send(new_symbols.clone()).await.is_err() {
                            break; // break the loop if there is no receiver
                        }
                        num_subscribed_of_last_client += new_symbols.len();
                        subscribed_symbols.append(&mut new_symbols);
                    }
                    if num_subscribed_of_last_client >= num_topics_per_connection {
                        panic!(
                            "The last connection has subscribed {} topics, which is more than {}, restarting the process",
                             num_subscribed_of_last_client, num_topics_per_connection,
                        ); // pm2 will restart the whole process
                    }
                }
            }
        }
    })
}

fn create_new_symbol_receiver_thread(
    exchange: String,
    msg_type: MessageType,
    market_type: MarketType,
    mut symbols_rx: tokio::sync::mpsc::Receiver<Vec<String>>,
    ws_client: Arc<dyn WSClient + Send + Sync>,
) -> tokio::task::JoinHandle<()> {
    tokio::task::spawn(async move {
        let exchange_clone = exchange;
        while let Some(new_symbols) = symbols_rx.recv().await {
            subscribe_with_lock(
                exchange_clone.clone(),
                market_type,
                msg_type,
                new_symbols,
                ws_client.clone(),
            )
            .await;
        }
    })
}

fn create_new_symbol_receiver_thread_candlestick(
    intervals: Vec<usize>,
    mut rx: tokio::sync::mpsc::Receiver<Vec<String>>,
    ws_client: Arc<dyn WSClient + Send + Sync>,
) -> tokio::task::JoinHandle<()> {
    tokio::task::spawn(async move {
        while let Some(new_symbols) = rx.recv().await {
            let new_symbol_interval_list = new_symbols
                .iter()
                .flat_map(|symbol| {
                    intervals
                        .clone()
                        .into_iter()
                        .map(move |interval| (symbol.clone(), interval))
                })
                .collect::<Vec<(String, usize)>>();
            ws_client
                .subscribe_candlestick(&new_symbol_interval_list)
                .await;
        }
    })
}

// create a thread to convert Sender<Message> Sender<String>
pub(crate) fn create_conversion_thread(
    exchange: String,
    msg_type: MessageType,
    market_type: MarketType,
    tx: Sender<Message>,
) -> Sender<String> {
    let (tx_raw, rx_raw) = std::sync::mpsc::channel();
    tokio::task::spawn(async move {
        for json in rx_raw {
            let msg = Message::new(exchange.clone(), market_type, msg_type, json);
            if tx.send(msg).is_err() {
                break; // break the loop if there is no receiver
            }
        }
    });
    tx_raw
}

// create a thread to call `crypto-msg-parser`
fn create_parser_thread(
    exchange: String,
    market_type: MarketType,
    tx: Sender<String>,
) -> Sender<String> {
    let (tx_raw, rx_raw) = std::sync::mpsc::channel::<String>();
    std::thread::spawn(move || {
        for json in rx_raw {
            let msg_type = crypto_msg_parser::get_msg_type(&exchange, &json);
            let parsed = match msg_type {
                MessageType::Trade => serde_json::to_string(
                    &crypto_msg_parser::parse_trade(&exchange, market_type, &json).unwrap(),
                )
                .unwrap(),
                MessageType::L2Event => {
                    let received_at = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis()
                        .try_into()
                        .unwrap();
                    serde_json::to_string(
                        &crypto_msg_parser::parse_l2(
                            &exchange,
                            market_type,
                            &json,
                            Some(received_at),
                        )
                        .unwrap(),
                    )
                    .unwrap()
                }
                _ => panic!("unknown msg type {}", msg_type),
            };
            if tx.send(parsed).is_err() {
                break; // break the loop if there is no receiver
            }
        }
    });
    tx_raw
}

async fn crawl_event_one_chunk(
    exchange: String,
    market_type: MarketType,
    msg_type: MessageType,
    ws_client: Option<Arc<dyn WSClient + Send + Sync>>,
    symbols: Vec<String>,
    tx: Sender<Message>,
) -> tokio::task::JoinHandle<()> {
    let ws_client = if let Some(ws_client) = ws_client {
        ws_client
    } else {
        let tx_clone = tx.clone();
        create_ws_client(&exchange, market_type, msg_type, tx_clone).await
    };

    {
        // fire and forget
        let exchange_clone = exchange.to_string();
        let ws_client_clone = ws_client.clone();
        tokio::task::spawn(async move {
            subscribe_with_lock(
                exchange_clone,
                market_type,
                msg_type,
                symbols,
                ws_client_clone,
            )
            .await;
        });
    }

    tokio::task::spawn(async move {
        ws_client.run().await;
        ws_client.close();
    })
}

pub(crate) async fn crawl_event(
    exchange: &str,
    msg_type: MessageType,
    market_type: MarketType,
    symbols: Option<&[String]>,
    tx: Sender<Message>,
) {
    let num_topics_per_connection = get_num_subscriptions_per_connection(exchange, market_type);
    let is_empty = match symbols {
        Some(list) => {
            if list.is_empty() {
                true
            } else {
                tokio::task::block_in_place(move || check_args(exchange, market_type, list));
                false
            }
        }
        None => true,
    };
    let automatic_symbol_discovery = is_empty;

    let real_symbols = if is_empty {
        tokio::task::block_in_place(move || fetch_symbols_retry(exchange, market_type))
    } else {
        symbols.unwrap().to_vec()
    };
    if real_symbols.is_empty() {
        error!("real_symbols is empty due to fetch_symbols_retry() failure");
        return;
    }

    // The stop channel is used by all tokio tasks
    let (stop_ch_tx, stop_ch_rx) = tokio::sync::broadcast::channel::<EmptyStruct>(1);

    // create a thread to discover new symbols
    let (tx_symbols, rx_symbols) = tokio::sync::mpsc::channel::<Vec<String>>(4);
    let symbol_discovery_thread = if automatic_symbol_discovery {
        let thread = create_symbol_discovery_thread(
            exchange.to_string(),
            market_type,
            real_symbols.clone(),
            stop_ch_rx,
            tx_symbols,
        );
        Some(thread)
    } else {
        None
    };

    // create a thread to convert Sender<String> to Sender<Message>
    if real_symbols.len() <= num_topics_per_connection {
        let ws_client = create_ws_client(exchange, market_type, msg_type, tx).await;
        subscribe_with_lock(
            exchange.to_string(),
            market_type,
            msg_type,
            real_symbols,
            ws_client.clone(),
        )
        .await;
        if automatic_symbol_discovery {
            create_new_symbol_receiver_thread(
                exchange.to_string(),
                msg_type,
                market_type,
                rx_symbols,
                ws_client.clone(),
            );
        }
        ws_client.run().await;
        ws_client.close();
    } else {
        // split to chunks
        let mut chunks: Vec<Vec<String>> = Vec::new();
        for i in (0..real_symbols.len()).step_by(num_topics_per_connection) {
            let chunk = real_symbols
                [i..(std::cmp::min(i + num_topics_per_connection, real_symbols.len()))]
                .to_vec();
            chunks.push(chunk);
        }
        debug!(
            "{} {} {}",
            real_symbols.len(),
            num_topics_per_connection,
            chunks.len(),
        );
        assert!(chunks.len() > 1);

        let mut last_ws_client = None;
        let mut handles = Vec::new();
        {
            let n = chunks.len();
            for (i, chunk) in chunks.into_iter().enumerate() {
                last_ws_client = if i == (n - 1) {
                    let tx_clone = tx.clone();
                    Some(create_ws_client(exchange, market_type, msg_type, tx_clone).await)
                } else {
                    None
                };
                let ret = crawl_event_one_chunk(
                    exchange.to_string(),
                    market_type,
                    msg_type,
                    last_ws_client.clone(),
                    chunk,
                    tx.clone(),
                );
                handles.push(ret.await);
            }
            drop(tx);
        }
        if automatic_symbol_discovery && last_ws_client.is_some() {
            create_new_symbol_receiver_thread(
                exchange.to_string(),
                msg_type,
                market_type,
                rx_symbols,
                last_ws_client.unwrap(),
            );
        }
        for handle in handles {
            if let Err(err) = handle.await {
                panic!("{}", err); // TODO: use tokio::task::JoinSet or futures::stream::FuturesUnordered
            }
        }
    };
    _ = stop_ch_tx.send(EmptyStruct {});
    if let Some(thread) = symbol_discovery_thread {
        _ = thread.await;
    }
}

// from 1m to 5m
fn get_candlestick_intervals(exchange: &str, market_type: MarketType) -> Vec<usize> {
    match exchange {
        "binance" => vec![60, 180, 300],
        "bybit" => vec![60, 180, 300],
        "deribit" => vec![60, 180, 300],
        "gate" => vec![10, 60, 300],
        "kucoin" => match market_type {
            MarketType::Spot => vec![60, 300], // Reduced to avoid Broken pipe (os error 32)
            _ => vec![60, 300],
        },
        "okx" => vec![60, 180, 300],
        "zb" => match market_type {
            MarketType::Spot => vec![60, 180, 300],
            MarketType::LinearSwap => vec![60, 300],
            _ => vec![60, 180, 300],
        },
        "zbg" => match market_type {
            MarketType::Spot => vec![60, 300],
            _ => vec![60, 180, 300],
        },
        _ => vec![60, 300],
    }
}

async fn crawl_candlestick_one_chunk(
    exchange: String,
    market_type: MarketType,
    ws_client: Option<Arc<dyn WSClient + Send + Sync>>,
    symbol_interval_list: Vec<(String, usize)>,
    tx: Sender<Message>,
) -> tokio::task::JoinHandle<()> {
    let ws_client = if let Some(ws_client) = ws_client {
        ws_client
    } else {
        let tx_clone = tx.clone();
        create_ws_client(&exchange, market_type, MessageType::Candlestick, tx_clone).await
    };

    {
        // fire and forget
        let ws_client_clone = ws_client.clone();
        tokio::task::spawn(async move {
            ws_client_clone
                .subscribe_candlestick(&symbol_interval_list)
                .await;
        });
    }

    tokio::task::spawn(async move {
        ws_client.run().await;
        ws_client.close();
    })
}

pub(crate) async fn crawl_candlestick_ext(
    exchange: &str,
    market_type: MarketType,
    symbol_interval_list: Option<&[(String, usize)]>,
    tx: Sender<Message>,
) {
    let num_topics_per_connection = get_num_subscriptions_per_connection(exchange, market_type);
    let is_empty = match symbol_interval_list {
        Some(list) => {
            if list.is_empty() {
                true
            } else {
                let symbols: Vec<String> = list.iter().map(|t| t.0.clone()).collect();
                tokio::task::block_in_place(move || check_args(exchange, market_type, &symbols));
                false
            }
        }
        None => true,
    };
    let automatic_symbol_discovery = is_empty;

    let symbol_interval_list: Vec<(String, usize)> = if is_empty {
        let symbols =
            tokio::task::block_in_place(move || fetch_symbols_retry(exchange, market_type));
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
        error!("symbol_interval_list is empty due to fetch_symbols_retry() failure");
        return;
    }
    let real_symbols: Vec<String> = symbol_interval_list.iter().map(|t| t.0.clone()).collect();
    let real_intervals: Vec<usize> = symbol_interval_list.iter().map(|t| t.1).collect();

    // The stop channel is used by all tokio tasks
    let (stop_ch_tx, stop_ch_rx) = tokio::sync::broadcast::channel::<EmptyStruct>(1);

    // create a thread to discover new symbols
    let (tx_symbols, rx_symbols) = tokio::sync::mpsc::channel::<Vec<String>>(4);
    let symbol_discovery_thread = if automatic_symbol_discovery {
        let thread = create_symbol_discovery_thread(
            exchange.to_string(),
            market_type,
            real_symbols,
            stop_ch_rx,
            tx_symbols,
        );
        Some(thread)
    } else {
        None
    };

    if symbol_interval_list.len() <= num_topics_per_connection {
        let ws_client = create_ws_client(exchange, market_type, MessageType::Candlestick, tx).await;
        ws_client.subscribe_candlestick(&symbol_interval_list).await;
        if automatic_symbol_discovery {
            create_new_symbol_receiver_thread_candlestick(
                real_intervals,
                rx_symbols,
                ws_client.clone(),
            );
        }
        ws_client.run().await;
        ws_client.close();
    } else {
        // split to chunks
        let mut chunks: Vec<Vec<(String, usize)>> = Vec::new();
        {
            for i in (0..symbol_interval_list.len()).step_by(num_topics_per_connection) {
                let chunk: Vec<(String, usize)> = symbol_interval_list
                    [i..(std::cmp::min(i + num_topics_per_connection, symbol_interval_list.len()))]
                    .to_vec();
                chunks.push(chunk);
            }
        }
        debug!(
            "{} {} {}",
            symbol_interval_list.len(),
            num_topics_per_connection,
            chunks.len(),
        );
        assert!(chunks.len() > 1);

        let mut last_ws_client = None;
        let mut handles = Vec::new();
        {
            let n = chunks.len();
            for (i, chunk) in chunks.into_iter().enumerate() {
                last_ws_client = if i == (n - 1) {
                    let tx_clone = tx.clone();
                    Some(
                        create_ws_client(exchange, market_type, MessageType::Candlestick, tx_clone)
                            .await,
                    )
                } else {
                    None
                };
                let ret = crawl_candlestick_one_chunk(
                    exchange.to_string(),
                    market_type,
                    last_ws_client.clone(),
                    chunk,
                    tx.clone(),
                );
                handles.push(ret.await);
            }
            drop(tx);
        }
        if automatic_symbol_discovery && last_ws_client.is_some() {
            create_new_symbol_receiver_thread_candlestick(
                real_intervals,
                rx_symbols,
                last_ws_client.unwrap(),
            );
        }
        for handle in handles {
            if let Err(err) = handle.await {
                panic!("{}", err); // TODO: use tokio::task::JoinSet or futures::stream::FuturesUnordered
            }
        }
    };
    _ = stop_ch_tx.send(EmptyStruct {});
    if let Some(thread) = symbol_discovery_thread {
        _ = thread.await;
    }
}
