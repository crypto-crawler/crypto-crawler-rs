mod utils;

use crypto_msg_parser::{extract_symbol, parse_l2, parse_trade, MarketType, TradeSide};

#[test]
fn trade() {
    let raw_msg = r#"{"channel": "live_trades_btcusd", "data": {"amount": 1e-08, "amount_str": "1E-8", "buy_order_id": 1341285759094784, "id": 158457579, "microtimestamp": "1616297318187000", "price": 57748.8, "price_str": "57748.80", "sell_order_id": 1341285698236416, "timestamp": "1616297318", "type": 0}, "event": "trade"}"#;
    let trade = &parse_trade("bitstamp", MarketType::Spot, raw_msg).unwrap()[0];

    crate::utils::check_trade_fields(
        "bitstamp",
        MarketType::Spot,
        "BTC/USD".to_string(),
        extract_symbol("bitstamp", MarketType::Spot, raw_msg).unwrap(),
        trade,
    );

    assert_eq!(trade.quantity_base, 1e-08);
    assert_eq!(trade.side, TradeSide::Buy);
}

#[test]
fn l2_orderbook_update() {
    let raw_msg = r#"{"data":{"timestamp":"1622520011","microtimestamp":"1622520011989838","bids":[["36653.62","0.75000000"]],"asks":[["36665.20","0.00000000"],["36669.76","0.75000000"]]},"channel":"diff_order_book_btcusd","event":"data"}"#;
    let orderbook = &parse_l2("bitstamp", MarketType::Spot, raw_msg, None).unwrap()[0];

    assert_eq!(orderbook.asks.len(), 2);
    assert_eq!(orderbook.bids.len(), 1);
    assert!(!orderbook.snapshot);

    crate::utils::check_orderbook_fields(
        "bitstamp",
        MarketType::Spot,
        "BTC/USD".to_string(),
        extract_symbol("bitstamp", MarketType::Spot, raw_msg).unwrap(),
        orderbook,
    );

    assert_eq!(orderbook.timestamp, 1622520011989);

    assert_eq!(orderbook.bids[0].price, 36653.62);
    assert_eq!(orderbook.bids[0].quantity_base, 0.75);
    assert_eq!(orderbook.bids[0].quantity_quote, 36653.62 * 0.75);

    assert_eq!(orderbook.asks[0].price, 36665.2);
    assert_eq!(orderbook.asks[0].quantity_base, 0.0);
    assert_eq!(orderbook.asks[0].quantity_quote, 0.0);

    assert_eq!(orderbook.asks[1].price, 36669.76);
    assert_eq!(orderbook.asks[1].quantity_base, 0.75);
    assert_eq!(orderbook.asks[1].quantity_quote, 36669.76 * 0.75);
}
