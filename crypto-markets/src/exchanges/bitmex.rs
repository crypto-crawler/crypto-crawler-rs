use super::utils::http_get;
use crate::{error::Result, Market, MarketType};

use serde::{Deserialize, Serialize};

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    let instruments = fetch_instruments(market_type)?;
    Ok(instruments
        .into_iter()
        .map(|x| x.symbol)
        .collect::<Vec<String>>())
}

pub(crate) fn fetch_markets(_market_type: MarketType) -> Result<Vec<Market>> {
    Ok(Vec::new())
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
    lotSize: i64,
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
    prevClosePrice: f64,
    limitDownPrice: Option<f64>,
    limitUpPrice: Option<f64>,
    bankruptLimitDownPrice: Option<f64>,
    bankruptLimitUpPrice: Option<f64>,
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
    prevPrice24h: f64,
    vwap: f64,
    highPrice: f64,
    lowPrice: f64,
    lastPrice: f64,
    lastPriceProtected: f64,
    lastTickDirection: String,
    lastChangePcnt: f64,
    bidPrice: f64,
    midPrice: f64,
    askPrice: f64,
    impactBidPrice: f64,
    impactMidPrice: f64,
    impactAskPrice: f64,
    hasLiquidity: bool,
    openInterest: i64,
    openValue: i64,
    fairMethod: String,
    fairBasisRate: f64,
    fairBasis: f64,
    fairPrice: f64,
    markMethod: String,
    markPrice: f64,
    indicativeTaxRate: Option<f64>,
    indicativeSettlePrice: f64,
    optionUnderlyingPrice: Option<f64>,
    settledPriceAdjustmentRate: Option<f64>,
    settledPrice: Option<f64>,
    timestamp: String,
}

fn fetch_instruments(market_type: MarketType) -> Result<Vec<Instrument>> {
    let text = http_get("https://www.bitmex.com/api/v1/instrument/active", None)?;
    let instruments: Vec<Instrument> = serde_json::from_str::<Vec<Instrument>>(&text)
        .unwrap()
        .into_iter()
        .filter(|x| x.state == "Open")
        .collect();

    let swap: Vec<Instrument> = instruments
        .iter()
        .filter(|x| x.expiry.is_none())
        .cloned()
        .collect();
    let futures: Vec<Instrument> = instruments
        .iter()
        .filter(|x| x.expiry.is_some())
        .cloned()
        .collect();

    // Check
    for x in instruments.iter() {
        assert_eq!(x.underlying, x.rootSymbol);
        assert_eq!("XBt".to_string(), x.settlCurrency);
    }
    for x in swap.iter() {
        assert_eq!("FundingRate", x.fairMethod.as_str());
        assert!(x.expiry.is_none());
        assert_eq!(x.symbol, format!("{}{}", x.underlying, x.quoteCurrency));
    }
    for x in futures.iter() {
        assert_eq!("ImpactMidPrice", x.fairMethod.as_str());
        assert!(x.expiry.is_some());
    }
    // Inverse
    for x in instruments.iter().filter(|x| x.isInverse) {
        assert_eq!("XBT".to_string(), x.rootSymbol);
        assert_eq!("USD".to_string(), x.quoteCurrency);
        assert_eq!("USD".to_string(), x.positionCurrency);
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
