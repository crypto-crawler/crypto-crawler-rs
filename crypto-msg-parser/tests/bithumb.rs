mod utils;

use crypto_market_type::MarketType;
use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_l2, parse_trade, TradeSide};
use crypto_msg_type::MessageType;

#[test]
fn trade() {
    let raw_msg = r#"{"code":"00006","data":[{"p":"59023.7500000000","s":"sell","symbol":"BTC-USDT","t":"1616271104","v":"0.002873","ver":"19894683"},{"p":"59017.5100000000","s":"sell","symbol":"BTC-USDT","t":"1616271104","v":"0.001587","ver":"19894682"}],"timestamp":1616271105098,"topic":"TRADE"}"#;
    let trades = &parse_trade("bithumb", MarketType::Spot, raw_msg).unwrap();

    assert_eq!(trades.len(), 2);

    for trade in trades.iter() {
        crate::utils::check_trade_fields(
            "bithumb",
            MarketType::Spot,
            "BTC/USDT".to_string(),
            extract_symbol("bithumb", MarketType::Spot, raw_msg).unwrap(),
            trade,
            raw_msg,
        );

        assert_eq!(trade.side, TradeSide::Sell);
    }
    assert_eq!(
        1616271105098,
        extract_timestamp("bithumb", MarketType::Spot, raw_msg, None).unwrap()
    );

    let raw_msg = r#"{"code":"00007","data":{"p":"1674.7700000000","symbol":"ETH-USDT","ver":"15186035","s":"buy","t":"1616487024","v":"0.065614"},"topic":"TRADE","timestamp":1616487024837}"#;
    let trades = &parse_trade("bithumb", MarketType::Spot, raw_msg).unwrap();

    assert_eq!(trades.len(), 1);
    let trade = &trades[0];

    assert_eq!(&trade.trade_id, "15186035");

    crate::utils::check_trade_fields(
        "bithumb",
        MarketType::Spot,
        "ETH/USDT".to_string(),
        extract_symbol("bithumb", MarketType::Spot, raw_msg).unwrap(),
        trade,
        raw_msg,
    );

    assert_eq!(trade.quantity_base, 0.065614);
    assert_eq!(trade.side, TradeSide::Buy);
}

#[test]
fn l2_orderbook_snapshot() {
    let raw_msg = r#"{"code":"00006","data":{"b":[["35909.4500000000","0.007308"],["35905.3800000000","0.015820"],["35898.7500000000","0.016811"]],"s":[["34578.8700000000","0.000000"],["35927.4900000000","0.019198"],["35934.6800000000","0.016004"]],"symbol":"BTC-USDT","ver":"509670288"},"timestamp":1622446974153,"topic":"ORDERBOOK"}"#;
    let orderbook = &parse_l2("bithumb", MarketType::Spot, raw_msg, None).unwrap()[0];

    assert_eq!(orderbook.asks.len(), 3);
    assert_eq!(orderbook.bids.len(), 3);
    assert!(orderbook.snapshot);

    crate::utils::check_orderbook_fields(
        "bithumb",
        MarketType::Spot,
        MessageType::L2Event,
        "BTC/USDT".to_string(),
        extract_symbol("bithumb", MarketType::Spot, raw_msg).unwrap(),
        orderbook,
        raw_msg,
    );
    assert_eq!(
        1622446974153,
        extract_timestamp("bithumb", MarketType::Spot, raw_msg, None).unwrap()
    );

    assert_eq!(orderbook.timestamp, 1622446974153);
    assert_eq!(orderbook.seq_id, Some(509670288));

    assert_eq!(orderbook.bids[0].price, 35909.45);
    assert_eq!(orderbook.bids[0].quantity_base, 0.007308);
    assert_eq!(orderbook.bids[0].quantity_quote, 35909.45 * 0.007308);

    assert_eq!(orderbook.bids[2].price, 35898.75);
    assert_eq!(orderbook.bids[2].quantity_base, 0.016811);
    assert_eq!(orderbook.bids[2].quantity_quote, 35898.75 * 0.016811);

    assert_eq!(orderbook.asks[0].price, 34578.87);
    assert_eq!(orderbook.asks[0].quantity_base, 0.0);
    assert_eq!(orderbook.asks[0].quantity_quote, 0.0);

    assert_eq!(orderbook.asks[2].price, 35934.68);
    assert_eq!(orderbook.asks[2].quantity_base, 0.016004);
    assert_eq!(orderbook.asks[2].quantity_quote, 35934.68 * 0.016004);
}

#[test]
fn l2_orderbook_update() {
    let raw_msg = r#"{"code":"00007","data":{"symbol":"BTC-USDT","b":[["34613.4400000000","0.015396"]],"ver":"509670303","s":[]},"topic":"ORDERBOOK","timestamp":1622446975394}"#;
    let orderbook = &parse_l2("bithumb", MarketType::Spot, raw_msg, None).unwrap()[0];

    assert_eq!(orderbook.asks.len(), 0);
    assert_eq!(orderbook.bids.len(), 1);
    assert!(!orderbook.snapshot);

    crate::utils::check_orderbook_fields(
        "bithumb",
        MarketType::Spot,
        MessageType::L2Event,
        "BTC/USDT".to_string(),
        extract_symbol("bithumb", MarketType::Spot, raw_msg).unwrap(),
        orderbook,
        raw_msg,
    );
    assert_eq!(
        1622446975394,
        extract_timestamp("bithumb", MarketType::Spot, raw_msg, None).unwrap()
    );

    assert_eq!(orderbook.timestamp, 1622446975394);
    assert_eq!(orderbook.seq_id, Some(509670303));

    assert_eq!(orderbook.bids[0].price, 34613.44);
    assert_eq!(orderbook.bids[0].quantity_base, 0.015396);
    assert_eq!(orderbook.bids[0].quantity_quote, 34613.44 * 0.015396);
}
