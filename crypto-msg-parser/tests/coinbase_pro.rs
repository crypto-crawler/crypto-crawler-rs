mod utils;

use crypto_msg_parser::{parse_trade, MarketType, TradeSide};

#[test]
fn trade() {
    let raw_msg = r#"{"type":"last_match","trade_id":147587438,"maker_order_id":"3dbaddb1-3dcf-4511-b81c-89450a56deb4","taker_order_id":"421f3aaa-dfdd-4192-805a-bb73462ea6db","side":"sell","size":"0.00031874","price":"57786.82","product_id":"BTC-USD","sequence":22962703070,"time":"2021-03-21T03:47:27.112041Z"}"#;
    let trade = &parse_trade("coinbase_pro", MarketType::Spot, raw_msg).unwrap()[0];

    crate::utils::check_trade_fields(
        "coinbase_pro",
        MarketType::Spot,
        "BTC/USD".to_string(),
        trade,
    );

    assert_eq!(trade.volume, trade.price * trade.quantity);
    assert_eq!(trade.side, TradeSide::Sell);
    println!("{}", serde_json::to_string_pretty(&trade).unwrap());
}
