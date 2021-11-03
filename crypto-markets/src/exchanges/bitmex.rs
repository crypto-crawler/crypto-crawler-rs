use std::collections::HashMap;

use super::utils::http_get;
use crate::{
    error::Result,
    market::{Fees, Precision},
    Market, MarketType,
};

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    let instruments = fetch_instruments(market_type)?;
    Ok(instruments
        .into_iter()
        .map(|x| x.symbol)
        .collect::<Vec<String>>())
}

pub(crate) fn fetch_markets(market_type: MarketType) -> Result<Vec<Market>> {
    let instruments = fetch_instruments(market_type)?;
    let markets: Vec<Market> = instruments
        .into_iter()
        .map(|x| {
            let info = serde_json::to_value(&x)
                .unwrap()
                .as_object()
                .unwrap()
                .clone();
            let base_id = x.underlying;
            let quote_id = x.quoteCurrency;
            let pair = crypto_pair::normalize_pair(&x.symbol, "bitmex").unwrap();
            let (base, quote) = {
                let v: Vec<&str> = pair.split('/').collect();
                (v[0].to_string(), v[1].to_string())
            };

            Market {
                exchange: "bitmex".to_string(),
                market_type,
                symbol: x.symbol,
                base_id,
                quote_id,
                base,
                quote,
                active: x.state == "Open",
                margin: true,
                fees: Fees {
                    maker: x.makerFee,
                    taker: x.takerFee,
                },
                precision: Precision {
                    tick_size: x.tickSize,
                    lot_size: x.lotSize,
                },
                quantity_limit: None,
                contract_value: Some((x.multiplier.abs() as f64) * 1e-8),
                delivery_date: if let Some(expiry) = x.expiry {
                    let timestamp = DateTime::parse_from_rfc3339(&expiry).unwrap();
                    Some(timestamp.timestamp_millis() as u64)
                } else {
                    None
                },
                info,
            }
        })
        .collect();
    Ok(markets)
}

#[derive(Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Instrument {
    symbol: String,
    rootSymbol: String,
    state: String,
    typ: String,
    listing: String,
    front: String,
    expiry: Option<String>,
    settle: Option<String>,
    listedSettle: Option<String>,
    relistInterval: Option<String>,
    inverseLeg: String,
    sellLeg: String,
    buyLeg: String,
    optionStrikePcnt: Option<f64>,
    optionStrikeRound: Option<f64>,
    optionStrikePrice: Option<f64>,
    optionMultiplier: Option<f64>,
    positionCurrency: String,
    underlying: String,
    quoteCurrency: String,
    underlyingSymbol: String,
    reference: String,
    referenceSymbol: String,
    calcInterval: Option<String>,
    publishInterval: Option<String>,
    publishTime: Option<String>,
    maxOrderQty: i64,
    maxPrice: f64,
    lotSize: f64,
    tickSize: f64,
    multiplier: i64,
    settlCurrency: String,
    underlyingToPositionMultiplier: Option<i64>,
    underlyingToSettleMultiplier: Option<i64>,
    quoteToSettleMultiplier: Option<i64>,
    isQuanto: bool,
    isInverse: bool,
    initMargin: f64,
    maintMargin: f64,
    riskLimit: i64,
    riskStep: i64,
    limit: Option<i64>,
    capped: bool,
    taxed: bool,
    deleverage: bool,
    makerFee: f64,
    takerFee: f64,
    settlementFee: f64,
    insuranceFee: f64,
    fundingBaseSymbol: String,
    fundingQuoteSymbol: String,
    fundingPremiumSymbol: String,
    fundingTimestamp: Option<String>,
    fundingInterval: Option<String>,
    fundingRate: Option<f64>,
    indicativeFundingRate: Option<f64>,
    rebalanceTimestamp: Option<String>,
    rebalanceInterval: Option<String>,
    openingTimestamp: String,
    closingTimestamp: String,
    sessionInterval: String,
    prevTotalVolume: i64,
    totalVolume: i64,
    volume: i64,
    volume24h: i64,
    prevTotalTurnover: i64,
    totalTurnover: i64,
    turnover: i64,
    turnover24h: i64,
    homeNotional24h: f64,
    foreignNotional24h: f64,
    lastTickDirection: String,
    hasLiquidity: bool,
    openInterest: i64,
    openValue: i64,
    fairMethod: String,
    markMethod: String,
    timestamp: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

fn fetch_instruments(market_type: MarketType) -> Result<Vec<Instrument>> {
    let text = http_get("https://www.bitmex.com/api/v1/instrument/active", None)?;
    let instruments: Vec<Instrument> = serde_json::from_str::<Vec<Instrument>>(&text)?
        .into_iter()
        .filter(|x| x.state == "Open")
        .collect();

    let swap: Vec<Instrument> = instruments
        .iter()
        .filter(|x| (&x.symbol[x.symbol.len() - 1..]).parse::<i32>().is_err())
        .cloned()
        .collect();
    let futures: Vec<Instrument> = instruments
        .iter()
        .filter(|x| (&x.symbol[x.symbol.len() - 1..]).parse::<i32>().is_ok())
        .cloned()
        .collect();

    // Check
    // for x in instruments.iter() {
    //     assert_eq!(x.underlying, x.rootSymbol);
    //     assert_eq!("XBt".to_string(), x.settlCurrency);
    // }
    for x in swap.iter() {
        assert_eq!("FundingRate", x.fairMethod.as_str());
        // assert!(x.expiry.is_none()); // TODO: BitMEX data is not correct, comment it for now
        assert_eq!(x.symbol, format!("{}{}", x.underlying, x.quoteCurrency));
    }
    for x in futures.iter() {
        assert_eq!("ImpactMidPrice", x.fairMethod.as_str());
        assert!(x.expiry.is_some());
    }
    // Inverse
    for x in instruments.iter().filter(|x| x.isInverse) {
        assert!(x.symbol.starts_with("XBT"));
        assert_eq!("XBT".to_string(), x.rootSymbol);
        // USD, EUR
        assert_eq!(x.quoteCurrency, x.positionCurrency);
    }
    // Quanto
    for x in instruments.iter().filter(|x| x.isQuanto) {
        assert!(x.positionCurrency.is_empty());
    }
    // Linear
    for x in instruments.iter().filter(|x| !x.isQuanto && !x.isInverse) {
        assert_eq!(x.positionCurrency, x.rootSymbol);
        assert_eq!(x.settlCurrency.to_uppercase(), x.quoteCurrency);
    }

    let filtered: Vec<Instrument> = match market_type {
        MarketType::Unknown => instruments,
        MarketType::InverseSwap => swap.iter().filter(|x| x.isInverse).cloned().collect(),
        MarketType::QuantoSwap => swap.iter().filter(|x| x.isQuanto).cloned().collect(),
        MarketType::LinearFuture => futures
            .iter()
            .filter(|x| !x.isInverse && !x.isQuanto)
            .cloned()
            .collect(),
        MarketType::InverseFuture => futures.iter().filter(|x| x.isInverse).cloned().collect(),
        MarketType::QuantoFuture => futures.iter().filter(|x| x.isQuanto).cloned().collect(),
        _ => panic!("Unsupported market_type: {}", market_type),
    };
    Ok(filtered)
}
