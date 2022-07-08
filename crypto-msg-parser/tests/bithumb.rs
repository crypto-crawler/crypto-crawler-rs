mod utils;

use crypto_market_type::MarketType;
use crypto_message::TradeSide;
use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_l2, parse_trade};
use crypto_msg_type::MessageType;

const EXCHANGE_NAME: &str = "bithumb";

#[test]
fn trade() {
    let raw_msg = r#"{"code":"00006","data":[{"p":"59023.7500000000","s":"sell","symbol":"BTC-USDT","t":"1616271104","v":"0.002873","ver":"19894683"},{"p":"59017.5100000000","s":"sell","symbol":"BTC-USDT","t":"1616271104","v":"0.001587","ver":"19894682"}],"timestamp":1616271105098,"topic":"TRADE"}"#;
    let trades = &parse_trade(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap();

    assert_eq!(trades.len(), 2);

    for trade in trades.iter() {
        crate::utils::check_trade_fields(
            EXCHANGE_NAME,
            MarketType::Spot,
            "BTC/USDT".to_string(),
            extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
            trade,
            raw_msg,
        );

        assert_eq!(trade.side, TradeSide::Sell);
    }
    assert_eq!(
        1616271105098,
        extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
            .unwrap()
            .unwrap()
    );

    let raw_msg = r#"{"code":"00007","data":{"p":"1674.7700000000","symbol":"ETH-USDT","ver":"15186035","s":"buy","t":"1616487024","v":"0.065614"},"topic":"TRADE","timestamp":1616487024837}"#;
    let trades = &parse_trade(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap();

    assert_eq!(trades.len(), 1);
    let trade = &trades[0];

    assert_eq!(&trade.trade_id, "15186035");

    crate::utils::check_trade_fields(
        EXCHANGE_NAME,
        MarketType::Spot,
        "ETH/USDT".to_string(),
        extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
        trade,
        raw_msg,
    );

    assert_eq!(trade.quantity_base, 0.065614);
    assert_eq!(trade.side, TradeSide::Buy);
}

#[test]
fn l2_orderbook_snapshot() {
    let raw_msg = r#"{"code":"00006","data":{"b":[["35909.4500000000","0.007308"],["35905.3800000000","0.015820"],["35898.7500000000","0.016811"]],"s":[["34578.8700000000","0.000000"],["35927.4900000000","0.019198"],["35934.6800000000","0.016004"]],"symbol":"BTC-USDT","ver":"509670288"},"timestamp":1622446974153,"topic":"ORDERBOOK"}"#;
    let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::Spot, raw_msg, None).unwrap()[0];

    assert_eq!(orderbook.asks.len(), 3);
    assert_eq!(orderbook.bids.len(), 3);
    assert!(orderbook.snapshot);

    crate::utils::check_orderbook_fields(
        EXCHANGE_NAME,
        MarketType::Spot,
        MessageType::L2Event,
        "BTC/USDT".to_string(),
        extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
        orderbook,
        raw_msg,
    );
    assert_eq!(
        1622446974153,
        extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
            .unwrap()
            .unwrap()
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
    let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::Spot, raw_msg, None).unwrap()[0];

    assert_eq!(orderbook.asks.len(), 0);
    assert_eq!(orderbook.bids.len(), 1);
    assert!(!orderbook.snapshot);

    crate::utils::check_orderbook_fields(
        EXCHANGE_NAME,
        MarketType::Spot,
        MessageType::L2Event,
        "BTC/USDT".to_string(),
        extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
        orderbook,
        raw_msg,
    );
    assert_eq!(
        1622446975394,
        extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
            .unwrap()
            .unwrap()
    );

    assert_eq!(orderbook.timestamp, 1622446975394);
    assert_eq!(orderbook.seq_id, Some(509670303));

    assert_eq!(orderbook.bids[0].price, 34613.44);
    assert_eq!(orderbook.bids[0].quantity_base, 0.015396);
    assert_eq!(orderbook.bids[0].quantity_quote, 34613.44 * 0.015396);
}

#[test]
fn ticker() {
    let raw_msg = r#"{"code":"00007","data":{"p":"-0.0512","symbol":"BTC-USDT","ver":"70013048","vol":"22818095.72371200","c":"29951.93","t":"22818095.72371200","v":"747.110521","h":"32252.34","l":"29250.95"},"topic":"TICKER","timestamp":1654161207269}"#;

    assert_eq!(
        1654161207269,
        extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
            .unwrap()
            .unwrap()
    );
    assert_eq!(
        "BTC-USDT",
        extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
    );
}

#[test]
fn l2_snapshot() {
    let raw_msg = r#"{"data":{"symbol":"BTC-USDT","b":[["30402.440000000000","0.001458"],["30370.910000000000","0.002482"],["30338.010000000000","0.000540"]],"ver":"876388569","s":[["30651.830000000000","0.003630"],["30686.780000000000","0.003420"],["30698.550000000000","0.004859"]]},"code":"0","msg":"success","timestamp":1654234202305,"startTime":null}"#;

    assert_eq!(
        1654234202305,
        extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
            .unwrap()
            .unwrap()
    );
    assert_eq!(
        "BTC-USDT",
        extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
    );
}
