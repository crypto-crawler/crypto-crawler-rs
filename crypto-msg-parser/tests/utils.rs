#![allow(dead_code)]
use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;

use crypto_message::{FundingRateMsg, OrderBookMsg, TradeMsg};
use crypto_msg_parser::{get_msg_type, round};

pub fn check_trade_fields(
    exchange: &str,
    market_type: MarketType,
    pair: String,
    symbol: String,
    trade: &TradeMsg,
    raw_msg: &str,
) {
    assert_eq!(trade.exchange, exchange);
    assert_eq!(trade.market_type, market_type);
    assert_eq!(trade.pair, pair);
    assert_eq!(trade.symbol, symbol);
    assert_eq!(trade.msg_type, MessageType::Trade);
    if [
        "binance", "bitget", "bitmex", "bybit", "deribit", "ftx", "huobi", "okex",
    ]
    .contains(&exchange)
    {
        assert_eq!(MessageType::Trade, get_msg_type(exchange, raw_msg));
    }
    assert!(trade.price > 0.0);
    assert!(trade.quantity_base > 0.0);
    assert!(trade.quantity_quote > 0.0);
    if exchange != "bitmex" {
        assert_eq!(
            round(trade.quantity_quote),
            round(trade.price * trade.quantity_base)
        );
    }
    assert!(!trade.trade_id.is_empty());
    assert_eq!(trade.timestamp.to_string().len(), 13);
}

pub fn check_orderbook_fields(
    exchange: &str,
    market_type: MarketType,
    msg_type: MessageType,
    pair: String,
    symbol: String,
    orderbook: &OrderBookMsg,
    raw_msg: &str,
) {
    assert_eq!(orderbook.exchange, exchange);
    assert_eq!(orderbook.market_type, market_type);
    assert_eq!(orderbook.msg_type, msg_type);
    assert_eq!(orderbook.pair, pair);
    assert_eq!(orderbook.symbol, symbol);
    if [
        "binance", "bitget", "bitmex", "bybit", "deribit", "ftx", "huobi", "okex",
    ]
    .contains(&exchange)
    {
        assert_eq!(msg_type, get_msg_type(exchange, raw_msg));
    }
    assert_eq!(orderbook.timestamp.to_string().len(), 13);

    for order in orderbook.asks.iter() {
        assert!(order.price > 0.0);
        assert!(order.quantity_base >= 0.0);
        assert!(order.quantity_quote >= 0.0);

        if let Some(quantity_contract) = order.quantity_contract {
            assert!(quantity_contract >= 0.0);
        }
    }
}

pub fn check_funding_rate_fields(
    exchange: &str,
    market_type: MarketType,
    funding_rate: &FundingRateMsg,
    raw_msg: &str,
) {
    assert_eq!(funding_rate.exchange, exchange);
    assert_eq!(funding_rate.market_type, market_type);
    // assert_eq!(funding_rate.pair, pair);
    assert_eq!(funding_rate.msg_type, MessageType::FundingRate);
    assert_eq!(MessageType::FundingRate, get_msg_type(exchange, raw_msg));
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
