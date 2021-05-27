mod utils;

use crypto_msg_parser::{parse_trade, MarketType, TradeSide};

#[test]
fn trade() {
    let raw_msg =
        r#"[321,[["57126.70000","0.02063928","1616333924.737428","b","m",""]],"trade","XBT/USD"]"#;
    let trade = &parse_trade("kraken", MarketType::Spot, raw_msg).unwrap()[0];

    crate::utils::check_trade_fields("kraken", MarketType::Spot, "BTC/USD".to_string(), trade);

    assert_eq!(trade.quantity_base, 0.02063928);
    assert_eq!(trade.side, TradeSide::Buy);
}
