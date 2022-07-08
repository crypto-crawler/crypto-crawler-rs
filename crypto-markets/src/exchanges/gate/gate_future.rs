use std::collections::HashMap;

use super::super::utils::http_get;
use crate::{error::Result, Fees, Market, Precision, QuantityLimit};

use crypto_market_type::MarketType;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// https://www.gateio.pro/docs/apiv4/zh_CN/#deliverycontract
#[derive(Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct FutureMarket {
    name: String,
    underlying: String,
    cycle: String,
    #[serde(rename = "type")]
    type_: String, // inverse, direct
    quanto_multiplier: String,
    leverage_min: String,
    leverage_max: String,
    maintenance_rate: String,
    mark_type: String, // internal, index
    maker_fee_rate: String,
    taker_fee_rate: String,
    order_price_round: String,
    mark_price_round: String,
    settle_fee_rate: String,
    expire_time: u64,
    order_size_min: f64,
    order_size_max: f64,
    in_delisting: bool,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// See https://www.gateio.pro/docs/apiv4/zh_CN/index.html#595cd9fe3c-2
fn fetch_future_markets_raw(settle: &str) -> Result<Vec<FutureMarket>> {
    let txt = http_get(
        format!("https://api.gateio.ws/api/v4/delivery/{}/contracts", settle).as_str(),
        None,
    )?;
    let markets = serde_json::from_str::<Vec<FutureMarket>>(&txt)?;
    Ok(markets
        .into_iter()
        .filter(|x| !x.in_delisting)
        .collect::<Vec<FutureMarket>>())
}

pub(super) fn fetch_inverse_future_symbols() -> Result<Vec<String>> {
    let symbols = fetch_future_markets_raw("btc")?
        .into_iter()
        .map(|m| m.name)
        .collect::<Vec<String>>();
    Ok(symbols)
}

pub(super) fn fetch_linear_future_symbols() -> Result<Vec<String>> {
    let symbols = fetch_future_markets_raw("usdt")?
        .into_iter()
        .map(|m| m.name)
        .collect::<Vec<String>>();
    Ok(symbols)
}

fn to_market(raw_market: &FutureMarket) -> Market {
    let pair = crypto_pair::normalize_pair(&raw_market.name, "gate").unwrap();
    let (base, quote) = {
        let v: Vec<&str> = pair.split('/').collect();
        (v[0].to_string(), v[1].to_string())
    };
    let (base_id, quote_id) = {
        let v: Vec<&str> = raw_market.underlying.split('_').collect();
        (v[0].to_string(), v[1].to_string())
    };
    let market_type = if raw_market.type_ == "inverse" {
        MarketType::InverseFuture
    } else if raw_market.type_ == "direct" {
        MarketType::LinearFuture
    } else {
        panic!("Unsupported type: {}", raw_market.type_);
    };
    let mut quanto_multiplier = raw_market.quanto_multiplier.parse::<f64>().unwrap();
    if raw_market.underlying == "BTC_USD" {
        assert_eq!(quanto_multiplier, 0.0);
        quanto_multiplier = 1.0;
    }
    assert!(quanto_multiplier > 0.0);

    Market {
        exchange: "gate".to_string(),
        market_type,
        symbol: raw_market.name.to_string(),
        base_id: base_id.clone(),
        quote_id: quote_id.clone(),
        settle_id: if market_type == MarketType::InverseFuture {
            Some(base_id)
        } else {
            Some(quote_id)
        },
        base: base.clone(),
        quote: quote.clone(),
        settle: if market_type == MarketType::InverseFuture {
            Some(base)
        } else {
            Some(quote)
        },
        active: !raw_market.in_delisting,
        margin: true,
        fees: Fees {
            maker: raw_market.maker_fee_rate.parse::<f64>().unwrap(),
            taker: raw_market.taker_fee_rate.parse::<f64>().unwrap(),
        },
        precision: Precision {
            tick_size: raw_market.order_price_round.parse::<f64>().unwrap(),
            lot_size: quanto_multiplier,
        },
        quantity_limit: Some(QuantityLimit {
            min: Some(raw_market.order_size_min),
            max: Some(raw_market.order_size_max),
            notional_min: None,
            notional_max: None,
        }),
        contract_value: Some(quanto_multiplier),
        delivery_date: Some((raw_market.expire_time * 1000) as u64),
        info: serde_json::to_value(raw_market)
            .unwrap()
            .as_object()
            .unwrap()
            .clone(),
    }
}

pub(super) fn fetch_inverse_future_markets() -> Result<Vec<Market>> {
    let markets = fetch_future_markets_raw("btc")?
        .into_iter()
        .map(|m| to_market(&m))
        .collect::<Vec<Market>>();
    Ok(markets)
}

pub(super) fn fetch_linear_future_markets() -> Result<Vec<Market>> {
    let markets = fetch_future_markets_raw("usdt")?
        .into_iter()
        .map(|m| to_market(&m))
        .collect::<Vec<Market>>();
    Ok(markets)
}
