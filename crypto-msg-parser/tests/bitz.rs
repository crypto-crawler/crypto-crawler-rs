mod utils;

use crypto_msg_parser::{parse_l2, parse_trade, MarketType, TradeSide};

#[test]
fn trade() {
    let raw_msg = r#"{"msgId":0,"params":{"symbol":"btc_usdt"},"action":"Pushdata.order","data":[{"id":"1616486110508","t":"15:55:10","T":1616486110,"p":"53874.97","n":"0.1310","s":"sell"},{"id":"1616486110006","t":"15:55:10","T":1616486110,"p":"53875.82","n":"0.1144","s":"buy"}],"time":1616486110921,"source":"sub-api"}"#;
    let trades = &parse_trade("bitz", MarketType::Spot, raw_msg).unwrap();

    assert_eq!(trades.len(), 2);

    for trade in trades.iter() {
        crate::utils::check_trade_fields("bitz", MarketType::Spot, "BTC/USDT".to_string(), trade);
    }

    assert_eq!(trades[0].side, TradeSide::Sell);
    assert_eq!(trades[0].quantity_base, 0.1310);

    assert_eq!(trades[1].side, TradeSide::Buy);
    assert_eq!(trades[1].quantity_base, 0.1144);
}

#[test]
fn l2_orderbook_update() {
    let raw_msg = r#"{"msgId":0,"params":{"symbol":"btc_usdt"},"action":"Pushdata.depth","data":{"asks":[["37520.67","0.8396","31502.3545"]],"bids":[["37328.48","0.0050","186.6424"],["37322.18","0.2462","9188.7207"]],"depthSerialNumber":329},"time":1622527417489,"source":"sub-api"}"#;
    let orderbook = &parse_l2("bitz", MarketType::Spot, raw_msg, None).unwrap()[0];

    assert_eq!(orderbook.asks.len(), 1);
    assert_eq!(orderbook.bids.len(), 2);
    assert!(!orderbook.snapshot);

    crate::utils::check_orderbook_fields(
        "bitz",
        MarketType::Spot,
        "BTC/USDT".to_string(),
        orderbook,
    );

    assert_eq!(orderbook.timestamp, 1622527417489);

    assert_eq!(orderbook.asks[0].price, 37520.67);
    assert_eq!(orderbook.asks[0].quantity_base, 0.8396);
    assert_eq!(orderbook.asks[0].quantity_quote, 31502.3545);

    assert_eq!(orderbook.bids[0].price, 37328.48);
    assert_eq!(orderbook.bids[0].quantity_base, 0.0050);
    assert_eq!(orderbook.bids[0].quantity_quote, 186.6424);

    assert_eq!(orderbook.bids[1].price, 37322.18);
    assert_eq!(orderbook.bids[1].quantity_base, 0.2462);
    assert_eq!(orderbook.bids[1].quantity_quote, 9188.7207);
}
