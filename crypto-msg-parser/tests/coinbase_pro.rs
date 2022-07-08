mod utils;

use chrono::prelude::*;
use crypto_market_type::MarketType;
use crypto_message::TradeSide;
use crypto_msg_parser::{extract_symbol, extract_timestamp, parse_l2, parse_trade};

use crypto_msg_type::MessageType;

const EXCHANGE_NAME: &str = "coinbase_pro";

#[test]
fn trade() {
    let raw_msg = r#"{"type":"last_match","trade_id":147587438,"maker_order_id":"3dbaddb1-3dcf-4511-b81c-89450a56deb4","taker_order_id":"421f3aaa-dfdd-4192-805a-bb73462ea6db","side":"sell","size":"0.00031874","price":"57786.82","product_id":"BTC-USD","sequence":22962703070,"time":"2021-03-21T03:47:27.112041Z"}"#;
    let trade = &parse_trade(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()[0];

    crate::utils::check_trade_fields(
        EXCHANGE_NAME,
        MarketType::Spot,
        "BTC/USD".to_string(),
        extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
        trade,
        raw_msg,
    );
    assert_eq!(
        1616298447112,
        extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
            .unwrap()
            .unwrap()
    );

    assert_eq!(trade.quantity_base, 0.00031874);
    assert_eq!(trade.side, TradeSide::Sell);
}

#[test]
fn l2_orderbook_snapshot() {
    let raw_msg = r#"{"type":"snapshot","product_id":"BTC-USD","asks":[["37212.77","0.05724592"],["37215.39","0.00900000"],["37215.69","0.09654865"]],"bids":[["37209.96","0.04016376"],["37209.32","0.00192256"],["37209.16","0.01130000"]]}"#;
    let received_at = Utc::now().timestamp_millis();
    let orderbook =
        &parse_l2(EXCHANGE_NAME, MarketType::Spot, raw_msg, Some(received_at)).unwrap()[0];

    assert_eq!(orderbook.asks.len(), 3);
    assert_eq!(orderbook.bids.len(), 3);
    assert!(orderbook.snapshot);

    crate::utils::check_orderbook_fields(
        EXCHANGE_NAME,
        MarketType::Spot,
        MessageType::L2Event,
        "BTC/USD".to_string(),
        extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
        orderbook,
        raw_msg,
    );
    assert_eq!(
        None,
        extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
    );
    assert_eq!(received_at, orderbook.timestamp);

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
    let orderbook = &parse_l2(EXCHANGE_NAME, MarketType::Spot, raw_msg, None).unwrap()[0];

    assert_eq!(orderbook.asks.len(), 0);
    assert_eq!(orderbook.bids.len(), 1);
    assert!(!orderbook.snapshot);

    crate::utils::check_orderbook_fields(
        EXCHANGE_NAME,
        MarketType::Spot,
        MessageType::L2Event,
        "BTC/USD".to_string(),
        extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap(),
        orderbook,
        raw_msg,
    );
    assert_eq!(
        1622624529048,
        extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
            .unwrap()
            .unwrap()
    );

    assert_eq!(orderbook.timestamp, 1622624529048);

    assert_eq!(orderbook.bids[0].price, 37378.26);
    assert_eq!(orderbook.bids[0].quantity_base, 0.0246);
    assert_eq!(orderbook.bids[0].quantity_quote, 37378.26 * 0.0246);
}

#[test]
fn l3_event() {
    let raw_msg = r#"{"price":"31572.35","order_id":"5816ff12-61fc-4ab0-877a-fdf88544a4ee","remaining_size":"0.23","type":"open","side":"sell","product_id":"BTC-USD","time":"2022-06-01T08:32:21.469151Z","sequence":38292760991}"#;

    assert_eq!(
        1654072341469,
        extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
            .unwrap()
            .unwrap()
    );
    assert_eq!(
        "BTC-USD",
        extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
    );
}

#[test]
fn ticker() {
    let raw_msg = r#"{"type":"ticker","sequence":38332655422,"product_id":"BTC-USD","price":"29940.91","open_24h":"31677.61","volume_24h":"27783.70216674","low_24h":"29308.01","high_24h":"31888","volume_30d":"778633.19135445","best_bid":"29940.90","best_ask":"29940.91","side":"buy","time":"2022-06-02T09:20:54.127011Z","trade_id":347875517,"last_size":"0.00061522"}"#;

    assert_eq!(
        1654161654127,
        extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg)
            .unwrap()
            .unwrap()
    );
    assert_eq!(
        "BTC-USD",
        extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
    );
}

#[test]
fn l2_snapshot() {
    let raw_msg = r#"{"bids": [["0.1135", "35", 1], ["0.1134", "20606.7", 5], ["0.1133", "41561.8", 8], ["0.1132", "51132.8", 4], ["0.1131", "745", 2]], "asks": [["0.1137", "10113.4", 4], ["0.1138", "49781.3", 6], ["0.1139", "34339.9", 6], ["0.114", "34409.1", 4], ["0.1141", "4126.6", 2]], "sequence": 406959136, "auction_mode": false, "auction": null}"#;

    assert_eq!(
        "NONE",
        extract_symbol(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
    );

    assert_eq!(
        None,
        extract_timestamp(EXCHANGE_NAME, MarketType::Spot, raw_msg).unwrap()
    );
}
