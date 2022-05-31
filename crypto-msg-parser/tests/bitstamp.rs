mod utils;

use crypto_market_type::MarketType;
use crypto_msg_parser::{
    extract_symbol, extract_timestamp, parse_l2, parse_l2_topk, parse_trade, TradeSide,
};
use crypto_msg_type::MessageType;

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
        raw_msg,
    );
    assert_eq!(
        1616297318187,
        extract_timestamp("bitstamp", MarketType::Spot, raw_msg)
            .unwrap()
            .unwrap()
    );

    assert_eq!(trade.price, 57748.8);
    assert_eq!(trade.quantity_base, 1e-08);
    assert_eq!(trade.quantity_quote, 57748.8 * 1e-08);
    assert_eq!(trade.quantity_contract, None);
    assert_eq!(trade.side, TradeSide::Buy);
}

#[test]
fn l2_event() {
    let raw_msg = r#"{"data":{"timestamp":"1622520011","microtimestamp":"1622520011989838","bids":[["36653.62","0.75000000"]],"asks":[["36665.20","0.00000000"],["36669.76","0.75000000"]]},"channel":"diff_order_book_btcusd","event":"data"}"#;
    let orderbook = &parse_l2("bitstamp", MarketType::Spot, raw_msg, None).unwrap()[0];

    assert_eq!(orderbook.asks.len(), 2);
    assert_eq!(orderbook.bids.len(), 1);
    assert!(!orderbook.snapshot);

    crate::utils::check_orderbook_fields(
        "bitstamp",
        MarketType::Spot,
        MessageType::L2Event,
        "BTC/USD".to_string(),
        extract_symbol("bitstamp", MarketType::Spot, raw_msg).unwrap(),
        orderbook,
        raw_msg,
    );
    assert_eq!(
        1622520011989,
        extract_timestamp("bitstamp", MarketType::Spot, raw_msg)
            .unwrap()
            .unwrap()
    );

    assert_eq!(orderbook.timestamp, 1622520011989);
    assert_eq!(orderbook.seq_id, None);
    assert_eq!(orderbook.prev_seq_id, None);

    assert_eq!(orderbook.bids[0].price, 36653.62);
    assert_eq!(orderbook.bids[0].quantity_base, 0.75);
    assert_eq!(orderbook.bids[0].quantity_quote, 36653.62 * 0.75);
    assert_eq!(orderbook.bids[0].quantity_contract, None);

    assert_eq!(orderbook.asks[0].price, 36665.2);
    assert_eq!(orderbook.asks[0].quantity_base, 0.0);
    assert_eq!(orderbook.asks[0].quantity_quote, 0.0);
    assert_eq!(orderbook.asks[0].quantity_contract, None);

    assert_eq!(orderbook.asks[1].price, 36669.76);
    assert_eq!(orderbook.asks[1].quantity_base, 0.75);
    assert_eq!(orderbook.asks[1].quantity_quote, 36669.76 * 0.75);
    assert_eq!(orderbook.asks[1].quantity_contract, None);
}

#[test]
fn l2_topk() {
    let raw_msg = r#"{"data":{"timestamp":"1653978373","microtimestamp":"1653978373164007","bids":[["31524.50","0.36400000"],["31521.05","0.23734197"],["31521.03","0.66028343"]],"asks":[["31535.44","0.31708837"],["31539.38","0.47520104"],["31543.37","0.01071471"]]},"channel":"order_book_btcusd","event":"data"}"#;
    let orderbook = &parse_l2_topk("bitstamp", MarketType::Spot, raw_msg, None).unwrap()[0];

    assert_eq!(orderbook.asks.len(), 3);
    assert_eq!(orderbook.bids.len(), 3);
    assert!(orderbook.snapshot);

    crate::utils::check_orderbook_fields(
        "bitstamp",
        MarketType::Spot,
        MessageType::L2TopK,
        "BTC/USD".to_string(),
        extract_symbol("bitstamp", MarketType::Spot, raw_msg).unwrap(),
        orderbook,
        raw_msg,
    );
    assert_eq!(
        1653978373164,
        extract_timestamp("bitstamp", MarketType::Spot, raw_msg)
            .unwrap()
            .unwrap()
    );

    assert_eq!(orderbook.timestamp, 1653978373164);
    assert_eq!(orderbook.seq_id, None);
    assert_eq!(orderbook.prev_seq_id, None);

    assert_eq!(orderbook.bids[0].price, 31524.50);
    assert_eq!(orderbook.bids[0].quantity_base, 0.364);
    assert_eq!(orderbook.bids[0].quantity_quote, 31524.50 * 0.364);
    assert_eq!(orderbook.bids[0].quantity_contract, None);

    assert_eq!(orderbook.bids[2].price, 31521.03);
    assert_eq!(orderbook.bids[2].quantity_base, 0.66028343);
    assert_eq!(orderbook.bids[2].quantity_quote, 31521.03 * 0.66028343);
    assert_eq!(orderbook.bids[2].quantity_contract, None);

    assert_eq!(orderbook.asks[0].price, 31535.44);
    assert_eq!(orderbook.asks[0].quantity_base, 0.31708837);
    assert_eq!(orderbook.asks[0].quantity_quote, 31535.44 * 0.31708837);
    assert_eq!(orderbook.asks[0].quantity_contract, None);

    assert_eq!(orderbook.asks[2].price, 31543.37);
    assert_eq!(orderbook.asks[2].quantity_base, 0.01071471);
    assert_eq!(orderbook.asks[2].quantity_quote, 31543.37 * 0.01071471);
    assert_eq!(orderbook.asks[2].quantity_contract, None);
}
