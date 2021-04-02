use crypto_market_type::MarketType;
use crypto_msg_parser::{FundingRateMsg, MessageType, TradeMsg};

pub fn check_trade_fields(exchange: &str, market_type: MarketType, pair: String, trade: &TradeMsg) {
    assert_eq!(trade.exchange, exchange);
    assert_eq!(trade.market_type, market_type);
    assert_eq!(trade.pair, pair);
    assert_eq!(trade.msg_type, MessageType::Trade);
    assert!(trade.price > 0.0);
    assert!(trade.quantity > 0.0);
    assert!(trade.volume > 0.0);
    assert!(!trade.trade_id.is_empty());
    assert_eq!(trade.timestamp.to_string().len(), 13);
}

pub fn check_funding_rate_fields(
    exchange: &str,
    market_type: MarketType,
    funding_rate: &FundingRateMsg,
) {
    assert_eq!(funding_rate.exchange, exchange);
    assert_eq!(funding_rate.market_type, market_type);
    // assert_eq!(funding_rate.pair, pair);
    assert_eq!(funding_rate.msg_type, MessageType::FundingRate);
    assert!(funding_rate.funding_rate > -1.0);
    assert!(funding_rate.funding_rate < 1.0);
    if exchange == "bitmex" {
        assert_eq!(funding_rate.funding_time % (4 * 3600000), 0);
    } else if exchange == "bitget" {
        assert_eq!(funding_rate.funding_time % 3600000, 0);
    } else {
        assert_eq!(funding_rate.funding_time % (8 * 3600000), 0);
    }
}
