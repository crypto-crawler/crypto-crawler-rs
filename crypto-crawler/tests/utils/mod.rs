use crypto_crawler::Message;
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

pub(crate) fn parse(msg: Message) -> bool {
    let skipped_exchanges = vec!["bitget", "zb"];
    if skipped_exchanges.contains(&msg.exchange.as_str()) {
        return true;
    }
    match msg.msg_type {
        MessageType::Trade => {
            crypto_msg_parser::parse_trade(&msg.exchange, msg.market_type, &msg.json).is_ok()
        }
        MessageType::L2Event => {
            match msg.market_type {
                // crypto-msg-parser doesn't support quanto contracts
                MarketType::QuantoSwap | MarketType::QuantoFuture => true,
                _ => crypto_msg_parser::parse_l2(
                    &msg.exchange,
                    msg.market_type,
                    &msg.json,
                    Some(msg.received_at as i64),
                )
                .is_ok(),
            }
        }
        MessageType::FundingRate => crypto_msg_parser::parse_funding_rate(
            &msg.exchange,
            msg.market_type,
            &msg.json,
            Some(msg.received_at as i64),
        )
        .is_ok(),
        _ => true,
    }
}

#[allow(unused_macros)]
macro_rules! test_one_symbol {
    ($crawl_func:ident, $exchange:expr, $market_type:expr, $symbol:expr, $msg_type:expr) => {{
        let (tx, rx) = std::sync::mpsc::channel();
        let symbols = vec![$symbol.to_string()];
        tokio::task::spawn(async move {
            $crawl_func($exchange, $market_type, Some(&symbols), tx).await;
        });

        let msg = rx.recv().unwrap();

        assert_eq!(msg.exchange, $exchange.to_string());
        assert_eq!(msg.market_type, $market_type);
        assert_eq!(msg.msg_type, $msg_type);

        assert!(tokio::task::block_in_place(move || parse(msg)));
    }};
}

#[allow(unused_macros)]
macro_rules! test_all_symbols {
    ($crawl_func:ident, $exchange:expr, $market_type:expr, $msg_type:expr) => {{
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::task::spawn(async move {
            $crawl_func($exchange, $market_type, None, tx).await;
        });

        let msg = rx.recv().unwrap();

        assert_eq!(msg.exchange, $exchange.to_string());
        assert_eq!(msg.market_type, $market_type);
        assert_eq!(msg.msg_type, $msg_type);

        assert!(tokio::task::block_in_place(move || parse(msg)));
    }};
}

#[allow(unused_macros)]
macro_rules! test_crawl_restful {
    ($crawl_func:ident, $exchange:expr, $market_type:expr, $symbol:expr, $msg_type:expr) => {{
        let (tx, rx) = std::sync::mpsc::channel();
        let symbols = vec![$symbol.to_string()];
        std::thread::spawn(move || {
            $crawl_func($exchange, $market_type, Some(&symbols), tx);
        });

        let msg = rx.recv().unwrap();

        assert_eq!(msg.exchange, $exchange.to_string());
        assert_eq!(msg.market_type, $market_type);
        assert_eq!(msg.msg_type, $msg_type);

        assert!(parse(msg));
    }};
}

#[allow(unused_macros)]
macro_rules! test_crawl_restful_all_symbols {
    ($crawl_func:ident, $exchange:expr, $market_type:expr, $msg_type:expr) => {{
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            $crawl_func($exchange, $market_type, None, tx);
        });

        let msg = rx.recv().unwrap();

        assert_eq!(msg.exchange, $exchange.to_string());
        assert_eq!(msg.market_type, $market_type);
        assert_eq!(msg.msg_type, $msg_type);

        assert!(parse(msg));
    }};
}

#[allow(unused_macros)]
macro_rules! gen_test_crawl_candlestick {
    ($exchange:expr, $market_type:expr) => {{
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::task::spawn(async move {
            crawl_candlestick($exchange, $market_type, None, tx).await;
        });

        let msg = rx.recv().unwrap();

        assert_eq!(msg.exchange, $exchange.to_string());
        assert_eq!(msg.market_type, $market_type);
        assert_eq!(msg.msg_type, MessageType::Candlestick);

        assert!(tokio::task::block_in_place(move || parse(msg)));
    }};
}

#[allow(unused_macros)]
macro_rules! gen_test_subscribe_symbol {
    ($exchange:expr, $market_type:expr, $symbol:expr) => {{
        let (tx, rx) = std::sync::mpsc::channel();
        tokio::task::spawn(async move {
            let msg_types = vec![MessageType::Trade, MessageType::L2Event];
            subscribe_symbol($exchange, $market_type, $symbol, &msg_types, tx).await;
        });

        let mut messages = Vec::new();
        for msg in rx {
            messages.push(msg);
            break;
        }
        assert!(!messages.is_empty());
    }};
}
