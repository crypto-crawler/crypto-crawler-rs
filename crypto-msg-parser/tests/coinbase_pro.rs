mod utils;

use chrono::prelude::*;
use crypto_msg_parser::{parse_l2, parse_trade, MarketType, TradeSide};

#[test]
fn trade() {
    let raw_msg = r#"{"type":"last_match","trade_id":147587438,"maker_order_id":"3dbaddb1-3dcf-4511-b81c-89450a56deb4","taker_order_id":"421f3aaa-dfdd-4192-805a-bb73462ea6db","side":"sell","size":"0.00031874","price":"57786.82","product_id":"BTC-USD","sequence":22962703070,"time":"2021-03-21T03:47:27.112041Z"}"#;
    let trade = &parse_trade("coinbase_pro", MarketType::Spot, raw_msg).unwrap()[0];

    crate::utils::check_trade_fields(
        "coinbase_pro",
        MarketType::Spot,
        "BTC/USD".to_string(),
        trade,
    );

    assert_eq!(trade.quantity_base, 0.00031874);
    assert_eq!(trade.side, TradeSide::Sell);
}

#[test]
fn l2_orderbook_snapshot() {
    let raw_msg = r#"{"type":"snapshot","product_id":"BTC-USD","asks":[["37212.77","0.05724592"],["37215.39","0.00900000"],["37215.69","0.09654865"]],"bids":[["37209.96","0.04016376"],["37209.32","0.00192256"],["37209.16","0.01130000"]]}"#;
    let orderbook = &parse_l2(
        "coinbase_pro",
        MarketType::Spot,
        raw_msg,
        Some(Utc::now().timestamp_millis()),
    )
    .unwrap()[0];

    assert_eq!(orderbook.asks.len(), 3);
    assert_eq!(orderbook.bids.len(), 3);
    assert!(orderbook.snapshot);

    crate::utils::check_orderbook_fields(
        "coinbase_pro",
        MarketType::Spot,
        "BTC/USD".to_string(),
        orderbook,
    );

    assert_eq!(orderbook.bids[0].price, 37209.96);
    assert_eq!(orderbook.bids[0].quantity_base, 0.04016376);
    assert_eq!(orderbook.bids[0].quantity_quote, 37209.96 * 0.04016376);

    assert_eq!(orderbook.bids[2].price, 37209.16);
    assert_eq!(orderbook.bids[2].quantity_base, 0.0113);
    assert_eq!(orderbook.bids[2].quantity_quote, 37209.16 * 0.0113);

    assert_eq!(orderbook.asks[0].price, 37212.77);
    assert_eq!(orderbook.asks[0].quantity_base, 0.05724592);
    assert_eq!(orderbook.asks[0].quantity_quote, 37212.77 * 0.05724592);

    assert_eq!(orderbook.asks[2].price, 37215.69);
    assert_eq!(orderbook.asks[2].quantity_base, 0.09654865);
    assert_eq!(orderbook.asks[2].quantity_quote, 37215.69 * 0.09654865);
}

#[test]
fn l2_orderbook_update() {
    let raw_msg = r#"{"type":"l2update","product_id":"BTC-USD","changes":[["buy","37378.26","0.02460000"]],"time":"2021-06-02T09:02:09.048568Z"}"#;
    let orderbook = &parse_l2("coinbase_pro", MarketType::Spot, raw_msg, None).unwrap()[0];

    assert_eq!(orderbook.asks.len(), 0);
    assert_eq!(orderbook.bids.len(), 1);
    assert!(!orderbook.snapshot);

    crate::utils::check_orderbook_fields(
        "coinbase_pro",
        MarketType::Spot,
        "BTC/USD".to_string(),
        orderbook,
    );

    assert_eq!(orderbook.timestamp, 1622624529048);

    assert_eq!(orderbook.bids[0].price, 37378.26);
    assert_eq!(orderbook.bids[0].quantity_base, 0.0246);
    assert_eq!(orderbook.bids[0].quantity_quote, 37378.26 * 0.0246);
}
