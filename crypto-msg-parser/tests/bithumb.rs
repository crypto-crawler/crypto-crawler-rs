mod utils;

use crypto_msg_parser::{parse_trade, MarketType, TradeSide};

#[test]
fn spot() {
    let raw_msg = r#"{"code":"00006","data":[{"p":"59023.7500000000","s":"sell","symbol":"BTC-USDT","t":"1616271104","v":"0.002873","ver":"19894683"},{"p":"59017.5100000000","s":"sell","symbol":"BTC-USDT","t":"1616271104","v":"0.001587","ver":"19894682"}],"timestamp":1616271105098,"topic":"TRADE"}"#;
    let trades = &parse_trade("bithumb", MarketType::Spot, raw_msg).unwrap();

    assert_eq!(trades.len(), 2);

    for trade in trades.iter() {
        crate::utils::check_trade_fields(
            "bithumb",
            MarketType::Spot,
            "BTC/USDT".to_string(),
            trade,
        );

        assert_eq!(trade.volume, trade.price * trade.quantity);
        assert_eq!(trade.side, TradeSide::Sell);
    }
}
