mod utils;

use crypto_msg_parser::{parse_trade, MarketType, TradeSide};

#[test]
fn trade() {
    let raw_msg = r#"{"channel": "live_trades_btcusd", "data": {"amount": 1e-08, "amount_str": "1E-8", "buy_order_id": 1341285759094784, "id": 158457579, "microtimestamp": "1616297318187000", "price": 57748.8, "price_str": "57748.80", "sell_order_id": 1341285698236416, "timestamp": "1616297318", "type": 0}, "event": "trade"}"#;
    let trade = &parse_trade("bitstamp", MarketType::Spot, raw_msg).unwrap()[0];

    crate::utils::check_trade_fields("bitstamp", MarketType::Spot, "BTC/USD".to_string(), trade);

    assert_eq!(trade.volume, trade.price * trade.quantity);
    assert_eq!(trade.side, TradeSide::Buy);
}
