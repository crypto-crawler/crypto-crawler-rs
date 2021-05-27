mod utils;

use crypto_msg_parser::{parse_trade, MarketType, TradeSide};

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
