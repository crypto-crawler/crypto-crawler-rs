use crypto_market_type::MarketType;
use crypto_msg_parser::{MessageType, TradeMsg};

pub fn check_trade_fields(exchange: &str, market_type: MarketType, pair: String, trade: &TradeMsg) {
    assert_eq!(trade.exchange, exchange);
    assert_eq!(trade.market_type, market_type);
    assert_eq!(trade.pair, pair);
    assert_eq!(trade.msg_type, MessageType::Trade);
    assert!(trade.price > 0.0);
    assert!(trade.quantity > 0.0);
    assert!(!trade.trade_id.is_empty());
}
