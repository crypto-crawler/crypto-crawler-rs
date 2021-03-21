mod utils;

use crypto_msg_parser::{parse_trade, MarketType, TradeSide};

#[test]
fn trade() {
    let raw_msg = r#"{"params":{"symbol":"btc_usdt","type":"order","_CDID":"100002","dataType":"1"},"action":"Pushdata.order","data":[{"id":"1616299593889","t":"12:06:33","T":1616299593,"p":"57625.22","n":"0.0890","s":"sell"},{"id":"1616299593297","t":"12:06:33","T":1616299593,"p":"57625.23","n":"0.0049","s":"buy"}],"time":1616299594334}"#;
    let trades = &parse_trade("bitz", MarketType::Spot, raw_msg).unwrap();

    assert_eq!(trades.len(), 2);

    for trade in trades.iter() {
        crate::utils::check_trade_fields("bitz", MarketType::Spot, "BTC/USDT".to_string(), trade);

        assert_eq!(trade.volume, trade.price * trade.quantity);
    }

    assert_eq!(trades[0].side, TradeSide::Sell);
    assert_eq!(trades[1].side, TradeSide::Buy);
}
