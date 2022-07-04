use std::collections::HashMap;

use super::utils::http_get;
use crate::{
    error::Result,
    market::{Fees, Precision, QuantityLimit},
    Market, MarketType,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    match market_type {
        MarketType::Spot => fetch_spot_symbols(),
        MarketType::LinearSwap => fetch_linear_swap_symbols(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}

#[derive(Serialize, Deserialize)]
struct RawMarket {
    pair: String,
    price_precision: i64,
    initial_margin: String,
    minimum_margin: String,
    maximum_order_size: String,
    minimum_order_size: String,
    expiration: String,
    margin: bool,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

fn fetch_raw_markets() -> Result<Vec<RawMarket>> {
    // can NOT use v2 API due to https://github.com/bitfinexcom/bitfinex-api-py/issues/95
    let text = http_get("https://api.bitfinex.com/v1/symbols_details", None)?;
    let markets = serde_json::from_str::<Vec<RawMarket>>(&text)?;
    let markets = markets
        .into_iter()
        .filter(|m| !m.pair.starts_with("test"))
        .collect::<Vec<RawMarket>>();
    Ok(markets)
}

pub(crate) fn fetch_markets(market_type: MarketType) -> Result<Vec<Market>> {
    let raw_markets = fetch_raw_markets()?;
    let raw_markets: Vec<RawMarket> = match market_type {
        MarketType::Spot => raw_markets
            .into_iter()
            .filter(|x| !x.pair.ends_with("f0"))
            .collect(),
        MarketType::LinearSwap => raw_markets
            .into_iter()
            .filter(|x| x.pair.ends_with("f0"))
            .collect(),
        _ => panic!("Unsupported market_type: {}", market_type),
    };
    let markets: Vec<Market> = raw_markets
        .into_iter()
        .map(|m| {
            let symbol = m.pair.to_uppercase();
            let pair = crypto_pair::normalize_pair(&symbol, "bitfinex").unwrap();
            let (base, quote) = {
                let v: Vec<&str> = pair.split('/').collect();
                (v[0].to_string(), v[1].to_string())
            };
            let (base_id, quote_id) = if symbol.contains(':') {
                let v: Vec<&str> = symbol.split(':').collect();
                (v[0].to_string(), v[1].to_string())
            } else {
                (
                    symbol[..(symbol.len() - 3)].to_string(),
                    symbol[(symbol.len() - 3)..].to_string(),
                )
            };
            Market {
                exchange: "bitfinex".to_string(),
                market_type,
                symbol: format!("t{}", symbol),
                base_id,
                quote_id: quote_id.clone(),
                settle_id: if market_type == MarketType::LinearSwap {
                    Some(quote_id)
                } else {
                    None
                },
                base,
                quote: quote.clone(),
                settle: if market_type == MarketType::LinearSwap {
                    Some(quote)
                } else {
                    None
                },
                active: true,
                margin: m.margin,
                // see https://www.bitfinex.com/fees
                fees: if market_type == MarketType::Spot {
                    Fees {
                        maker: 0.001,
                        taker: 0.002,
                    }
                } else {
                    Fees {
                        maker: -0.0002,
                        taker: 0.00075,
                    }
                },
                precision: Precision {
                    tick_size: 1.0 / (10_i64.pow(m.price_precision as u32) as f64),
                    lot_size: 1.0 / (10_i64.pow(8_u32) as f64),
                },
                quantity_limit: Some(QuantityLimit {
                    min: m.minimum_order_size.parse::<f64>().ok(),
                    max: Some(m.maximum_order_size.parse::<f64>().unwrap()),
                    notional_min: None,
                    notional_max: None,
                }),
                contract_value: if market_type == MarketType::Spot {
                    None
                } else {
                    Some(1.0)
                },
                delivery_date: None,
                info: serde_json::to_value(&m)
                    .unwrap()
                    .as_object()
                    .unwrap()
                    .clone(),
            }
        })
        .collect();
    Ok(markets)
}

// see <https://docs.bitfinex.com/reference#rest-public-conf>
fn fetch_spot_symbols() -> Result<Vec<String>> {
    let text = http_get(
        "https://api-pub.bitfinex.com/v2/conf/pub:list:pair:exchange",
        None,
    )?;
    let pairs = serde_json::from_str::<Vec<Vec<String>>>(&text)?;
    let symbols = pairs[0]
        .iter()
        .filter(|x| !x.starts_with("TEST"))
        .map(|p| format!("t{}", p))
        .collect::<Vec<String>>();
    Ok(symbols)
}

// see <https://docs.bitfinex.com/reference#rest-public-conf>
fn fetch_linear_swap_symbols() -> Result<Vec<String>> {
    let text = http_get(
        "https://api-pub.bitfinex.com/v2/conf/pub:list:pair:futures",
        None,
    )?;
    let pairs = serde_json::from_str::<Vec<Vec<String>>>(&text)?;
    let symbols = pairs[0]
        .iter()
        .filter(|x| !x.starts_with("TEST"))
        .map(|p| format!("t{}", p))
        .collect::<Vec<String>>();
    Ok(symbols)
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::{
        super::utils::http_get, fetch_linear_swap_symbols, fetch_raw_markets, fetch_spot_symbols,
    };
    use crate::error::Result;

    fn _fetch_symbols(url: &str) -> Result<Vec<String>> {
        let text = http_get(url, None)?;
        let arr = serde_json::from_str::<Vec<Value>>(&text)?;
        let arr = serde_json::from_value::<Vec<Value>>(arr[0].clone())?;
        let symbols = arr
            .iter()
            .map(|p| format!("t{}", p[0].as_str().unwrap()))
            .filter(|x| !x.starts_with("tTEST"))
            .collect::<Vec<String>>();
        Ok(symbols)
    }

    fn _fetch_spot_symbols() -> Result<Vec<String>> {
        _fetch_symbols("https://api-pub.bitfinex.com/v2/conf/pub:info:pair")
    }

    fn _fetch_linear_swap_symbols() -> Result<Vec<String>> {
        _fetch_symbols("https://api-pub.bitfinex.com/v2/conf/pub:info:pair:futures")
    }

    #[test]
    fn test_spot_symbols() {
        let mut symbols1 = _fetch_spot_symbols().unwrap();
        let symbols2 = fetch_spot_symbols().unwrap();
        assert_eq!(symbols1, symbols2);

        let mut symbols3: Vec<String> = fetch_raw_markets()
            .unwrap()
            .into_iter()
            .map(|m| format!("t{}", m.pair.to_uppercase()))
            .filter(|x| !x.ends_with("F0"))
            .collect();
        symbols1.sort();
        symbols3.sort();
        // assert_eq!(symbols1, symbols3); // sometimes symbols3 has extra symbols that don't exist in symbols1
    }

    #[test]
    fn test_linear_swap_symbols() {
        let mut symbols1 = _fetch_linear_swap_symbols().unwrap();
        let symbols2 = fetch_linear_swap_symbols().unwrap();
        assert_eq!(symbols1, symbols2);

        let mut symbols3: Vec<String> = fetch_raw_markets()
            .unwrap()
            .into_iter()
            .map(|m| format!("t{}", m.pair.to_uppercase()))
            .filter(|x| x.ends_with("F0"))
            .collect();
        symbols1.sort();
        symbols3.sort();
        assert_eq!(symbols1, symbols3);
    }
}
