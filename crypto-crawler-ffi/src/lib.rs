#![allow(clippy::not_unsafe_ptr_arg_deref)]
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
    sync::{Arc, Mutex},
};

/// Market type.
#[repr(C)]
#[derive(Copy, Clone)]
pub enum MarketType {
    Spot,
    LinearFuture,
    InverseFuture,
    LinearSwap,
    InverseSwap,
    Option,

    QuantoFuture,
    QuantoSwap,
}

impl MarketType {
    // Converts C MarketType to Rust MarketType
    pub fn to_rust(&self) -> crypto_crawler::MarketType {
        match *self {
            MarketType::Spot => crypto_crawler::MarketType::Spot,
            MarketType::LinearFuture => crypto_crawler::MarketType::LinearFuture,
            MarketType::InverseFuture => crypto_crawler::MarketType::InverseFuture,
            MarketType::LinearSwap => crypto_crawler::MarketType::LinearSwap,
            MarketType::InverseSwap => crypto_crawler::MarketType::InverseSwap,
            MarketType::Option => crypto_crawler::MarketType::Option,
            MarketType::QuantoFuture => crypto_crawler::MarketType::QuantoFuture,
            MarketType::QuantoSwap => crypto_crawler::MarketType::QuantoSwap,
        }
    }

    // Converts Rust MarketType to C MarketType
    pub fn from_rust(market_type: crypto_crawler::MarketType) -> Self {
        match market_type {
            crypto_crawler::MarketType::Spot => MarketType::Spot,
            crypto_crawler::MarketType::LinearFuture => MarketType::LinearFuture,
            crypto_crawler::MarketType::InverseFuture => MarketType::InverseFuture,
            crypto_crawler::MarketType::LinearSwap => MarketType::LinearSwap,
            crypto_crawler::MarketType::InverseSwap => MarketType::InverseSwap,
            crypto_crawler::MarketType::Option => MarketType::Option,
            crypto_crawler::MarketType::QuantoFuture => MarketType::QuantoFuture,
            crypto_crawler::MarketType::QuantoSwap => MarketType::QuantoSwap,
        }
    }
}

/// The type of a message
#[repr(C)]
pub enum MessageType {
    Trade,
    L2Event,
    L2Snapshot,
    L3Event,
    L3Snapshot,
    BBO,
    Ticker,
    Candlestick,
}

impl MessageType {
    // Converts Rust MessageType to C MessageType
    pub fn from_rust(msg_type: crypto_crawler::MessageType) -> Self {
        match msg_type {
            crypto_crawler::MessageType::Trade => MessageType::Trade,
            crypto_crawler::MessageType::L2Event => MessageType::L2Event,
            crypto_crawler::MessageType::L2Snapshot => MessageType::L2Snapshot,
            crypto_crawler::MessageType::L3Event => MessageType::L3Event,
            crypto_crawler::MessageType::L3Snapshot => MessageType::L3Snapshot,
            crypto_crawler::MessageType::BBO => MessageType::BBO,
            crypto_crawler::MessageType::Ticker => MessageType::Ticker,
            crypto_crawler::MessageType::Candlestick => MessageType::Candlestick,
        }
    }
}

/// Message represents messages received by crawlers.
#[repr(C)]
pub struct Message {
    /// The exchange name, unique for each exchage
    pub exchange: *const c_char,
    /// Market type
    pub market_type: MarketType,
    /// Message type
    pub msg_type: MessageType,
    /// Exchange specific symbol, used by RESTful APIs and websocket
    pub symbol: *const c_char,
    /// Unix timestamp in milliseconds
    pub received_at: u64,
    /// the original message
    pub json: *const c_char,
}

// Converts an array of symbols from C to rust
fn convert_symbols(symbols: *const *const c_char, num_symbols: usize) -> Vec<String> {
    let mut arr = Vec::<String>::new();
    if num_symbols > 0 {
        for i in 0..num_symbols {
            let c_str = unsafe {
                let symbol_ptr: *const c_char = *(symbols.add(i));
                assert!(!symbol_ptr.is_null());
                CStr::from_ptr(symbol_ptr)
            };
            arr.push(c_str.to_str().unwrap().to_string());
        }
    }
    assert_eq!(arr.len(), num_symbols);
    arr
}

fn process_msg(on_msg: extern "C" fn(*const Message), msg: crypto_crawler::Message) {
    let exchange_cstring = CString::new(msg.exchange).unwrap();
    let symbol_cstring = CString::new(msg.symbol).unwrap();
    let json_cstring = CString::new(msg.json).unwrap();

    let msg_ffi = Message {
        exchange: exchange_cstring.as_ptr(),
        market_type: MarketType::from_rust(msg.market_type),
        msg_type: MessageType::from_rust(msg.msg_type),
        symbol: symbol_cstring.as_ptr(),
        received_at: msg.received_at,
        json: json_cstring.as_ptr(),
    };
    on_msg(&msg_ffi);
}

/// Crawl realtime trades.
///
/// ## Arguments
///
/// - `exchange` The exchange name, can not be null
/// - `market_type` The market type
/// - `symbols` Symbols to crawl
/// - `num_symbols` Number of symbols, 0 means all symbols in the `market_type`
/// - `on_msg` The callback function to process messages
/// - `duration` How many seconds to run, only useful in testing
#[no_mangle]
pub extern "C" fn crawl_trade(
    exchange: *const c_char,
    market_type: MarketType,
    symbols: *const *const c_char,
    num_symbols: usize,
    on_msg: extern "C" fn(*const Message),
    duration: u64,
) {
    let c_str = unsafe {
        assert!(!exchange.is_null());
        CStr::from_ptr(exchange)
    };
    let exchange_rust = c_str.to_str().unwrap();

    let symbols_rust = convert_symbols(symbols, num_symbols);

    let on_msg_ext = Arc::new(Mutex::new(move |msg: crypto_crawler::Message| {
        process_msg(on_msg, msg);
    }));

    crypto_crawler::crawl_trade(
        exchange_rust,
        market_type.to_rust(),
        if symbols_rust.is_empty() {
            None
        } else {
            Some(&symbols_rust)
        },
        on_msg_ext,
        if duration > 0 { Some(duration) } else { None },
    );
}

/// Crawl level2 orderbook update events.
#[no_mangle]
pub extern "C" fn crawl_l2_event(
    exchange: *const c_char,
    market_type: MarketType,
    symbols: *const *const c_char,
    num_symbols: usize,
    on_msg: extern "C" fn(*const Message),
    duration: u64,
) {
    let c_str = unsafe {
        assert!(!exchange.is_null());
        CStr::from_ptr(exchange)
    };
    let exchange_rust = c_str.to_str().unwrap();

    let symbols_rust = convert_symbols(symbols, num_symbols);

    let on_msg_ext = Arc::new(Mutex::new(move |msg: crypto_crawler::Message| {
        process_msg(on_msg, msg);
    }));

    crypto_crawler::crawl_l2_event(
        exchange_rust,
        market_type.to_rust(),
        if symbols_rust.is_empty() {
            None
        } else {
            Some(&symbols_rust)
        },
        on_msg_ext,
        if duration > 0 { Some(duration) } else { None },
    );
}

/// Crawl level2 orderbook snapshots through RESTful APIs.
#[no_mangle]
pub extern "C" fn crawl_l2_snapshot(
    exchange: *const c_char,
    market_type: MarketType,
    symbols: *const *const c_char,
    num_symbols: usize,
    on_msg: extern "C" fn(*const Message),
    interval: u64,
    duration: u64,
) {
    let c_str = unsafe {
        assert!(!exchange.is_null());
        CStr::from_ptr(exchange)
    };
    let exchange_rust = c_str.to_str().unwrap();

    let symbols_rust = convert_symbols(symbols, num_symbols);

    let on_msg_ext = Arc::new(Mutex::new(move |msg: crypto_crawler::Message| {
        process_msg(on_msg, msg);
    }));

    crypto_crawler::crawl_l2_snapshot(
        exchange_rust,
        market_type.to_rust(),
        if symbols_rust.is_empty() {
            None
        } else {
            Some(&symbols_rust)
        },
        on_msg_ext,
        Some(interval),
        if duration > 0 { Some(duration) } else { None },
    );
}

/// Crawl level3 orderbook update events.
#[no_mangle]
pub extern "C" fn crawl_l3_event(
    exchange: *const c_char,
    market_type: MarketType,
    symbols: *const *const c_char,
    num_symbols: usize,
    on_msg: extern "C" fn(*const Message),
    duration: u64,
) {
    let c_str = unsafe {
        assert!(!exchange.is_null());
        CStr::from_ptr(exchange)
    };
    let exchange_rust = c_str.to_str().unwrap();

    let symbols_rust = convert_symbols(symbols, num_symbols);

    let on_msg_ext = Arc::new(Mutex::new(move |msg: crypto_crawler::Message| {
        process_msg(on_msg, msg);
    }));

    crypto_crawler::crawl_l3_event(
        exchange_rust,
        market_type.to_rust(),
        if symbols_rust.is_empty() {
            None
        } else {
            Some(&symbols_rust)
        },
        on_msg_ext,
        if duration > 0 { Some(duration) } else { None },
    );
}

/// Crawl level3 orderbook snapshots through RESTful APIs.
#[no_mangle]
pub extern "C" fn crawl_l3_snapshot(
    exchange: *const c_char,
    market_type: MarketType,
    symbols: *const *const c_char,
    num_symbols: usize,
    on_msg: extern "C" fn(*const Message),
    interval: u64,
    duration: u64,
) {
    let c_str = unsafe {
        assert!(!exchange.is_null());
        CStr::from_ptr(exchange)
    };
    let exchange_rust = c_str.to_str().unwrap();

    let symbols_rust = convert_symbols(symbols, num_symbols);

    let on_msg_ext = Arc::new(Mutex::new(move |msg: crypto_crawler::Message| {
        process_msg(on_msg, msg);
    }));

    crypto_crawler::crawl_l3_snapshot(
        exchange_rust,
        market_type.to_rust(),
        if symbols_rust.is_empty() {
            None
        } else {
            Some(&symbols_rust)
        },
        on_msg_ext,
        Some(interval),
        if duration > 0 { Some(duration) } else { None },
    );
}
