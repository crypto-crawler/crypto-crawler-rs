use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crypto_market_type::MarketType;
use fslock::LockFile;
use lazy_static::lazy_static;

lazy_static! {
    static ref EXCHANGES: &'static [&'static str] = &[
        "binance",
        "bitfinex",
        "bitget",
        "bithumb",
        "bitmex",
        "bitstamp",
        "bitz",
        "bybit",
        "coinbase_pro",
        "deribit",
        "dydx",
        "ftx",
        "gate",
        "huobi",
        "kraken",
        "kraken_futures",
        "kucoin",
        "mxc",
        "okex",
        "zbg",
    ];
    pub(crate) static ref REST_LOCKS: HashMap<String, HashMap<MarketType, Arc<Mutex<LockFile>>>> =
        create_all_lock_files("rest");
    pub(crate) static ref WS_LOCKS: HashMap<String, HashMap<MarketType, Arc<Mutex<LockFile>>>> =
        create_all_lock_files("ws");
}

/// Markets with the same endpoint will have the same file name.
fn get_lock_file_name(exchange: &str, market_type: MarketType, prefix: &str) -> String {
    let filename = match exchange {
        "binance" => match market_type {
            MarketType::InverseSwap | MarketType::InverseFuture => {
                "binance_inverse.lock".to_string()
            }
            MarketType::LinearSwap | MarketType::LinearFuture => "binance_linear.lock".to_string(),
            MarketType::Spot => "binance_spot.lock".to_string(),
            MarketType::EuropeanOption => "binance_option.lock".to_string(),
            _ => panic!("Unknown market_type {} of {}", market_type, exchange),
        },
        "bitget" => match market_type {
            MarketType::InverseSwap | MarketType::LinearSwap => "bitget_swap.lock".to_string(),
            MarketType::Spot => "bitget_spot.lock".to_string(),
            _ => panic!("Unknown market_type {} of {}", market_type, exchange),
        },
        "bitmex" => "bitmex.lock".to_string(),
        "bitz" => match market_type {
            MarketType::InverseSwap | MarketType::LinearSwap => "bitz_swap.lock".to_string(),
            MarketType::Spot => "bitz_spot.lock".to_string(),
            _ => panic!("Unknown market_type {} of {}", market_type, exchange),
        },
        "deribit" => "deribit.lock".to_string(),
        "ftx" => "ftx.lock".to_string(),
        "gate" => match market_type {
            MarketType::InverseSwap | MarketType::LinearSwap => "gate_swap.lock".to_string(),
            MarketType::InverseFuture | MarketType::LinearFuture => "gate_future.lock".to_string(),
            MarketType::Spot => "gate_spot.lock".to_string(),
            _ => panic!("Unknown market_type {} of {}", market_type, exchange),
        },
        "kucoin" => {
            if prefix == "ws" {
                "kucoin.lock".to_string()
            } else {
                match market_type {
                    MarketType::InverseSwap
                    | MarketType::LinearSwap
                    | MarketType::InverseFuture => "kucoin_swap.lock".to_string(),
                    MarketType::Spot => "kucoin_spot.lock".to_string(),
                    MarketType::Unknown => "kucoin_unknown.lock".to_string(), // for OpenInterest
                    _ => panic!("Unknown market_type {} of {}", market_type, exchange),
                }
            }
        }
        "mxc" => match market_type {
            MarketType::InverseSwap | MarketType::LinearSwap => "mxc_swap.lock".to_string(),
            MarketType::Spot => "mxc_spot.lock".to_string(),
            _ => panic!("Unknown market_type {} of {}", market_type, exchange),
        },
        "okex" => "okex.lock".to_string(),
        "zbg" => match market_type {
            MarketType::InverseSwap | MarketType::LinearSwap => "zbg_swap.lock".to_string(),
            MarketType::Spot => "zbg_spot.lock".to_string(),
            _ => panic!("Unknown market_type {} of {}", market_type, exchange),
        },
        _ => format!("{}.{}.lock", exchange, market_type),
    };
    format!("{}.{}", prefix, filename)
}

fn create_lock_file(filename: &str) -> Arc<Mutex<LockFile>> {
    let mut dir = if std::env::var("DATA_DIR").is_ok() {
        std::path::Path::new(std::env::var("DATA_DIR").unwrap().as_str()).join("locks")
    } else {
        std::env::temp_dir().join("locks")
    };
    let _ = std::fs::create_dir_all(&dir);
    dir.push(filename);
    Arc::new(Mutex::new(LockFile::open(dir.as_path()).unwrap()))
}

fn create_all_lock_files(
    prefix: &str,
) -> HashMap<String, HashMap<MarketType, Arc<Mutex<LockFile>>>> {
    // filename -> lock
    let mut cache: HashMap<String, Arc<Mutex<LockFile>>> = HashMap::new();
    let mut result: HashMap<String, HashMap<MarketType, Arc<Mutex<LockFile>>>> = HashMap::new();
    for exchange in EXCHANGES.iter() {
        let m = result
            .entry(exchange.to_string())
            .or_insert_with(HashMap::new);
        let mut market_types = crypto_market_type::get_market_types(exchange);
        if *exchange == "bitmex" {
            market_types.push(MarketType::Unknown);
        }
        if prefix == "rest" {
            if *exchange == "ftx" || *exchange == "kucoin" {
                market_types.push(MarketType::Unknown); // for OpenInterest
            }
        }
        for market_type in market_types {
            let filename = get_lock_file_name(exchange, market_type, prefix);
            let lock_file = cache
                .entry(filename.clone())
                .or_insert_with(|| create_lock_file(&filename));
            m.insert(market_type, lock_file.clone());
        }
    }
    result
}
