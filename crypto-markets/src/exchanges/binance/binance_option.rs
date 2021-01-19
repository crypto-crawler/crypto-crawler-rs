use super::utils::binance_http_get;
use crate::{error::Result, market::*, Market, MarketType};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
#[derive(Serialize, Deserialize)]
struct BinanceResponse<T: Sized> {
    symbols: Vec<T>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct OptionMarket {
    id: i64,
    contractId: i64,
    underlying: String,
    quoteAsset: String,
    symbol: String,
    unit: i64,
    minQty: f64,
    maxQty: f64,
    priceScale: i64,
    quantityScale: i64,
    side: String,
    makerFeeRate: f64,
    takerFeeRate: f64,
    expiryDate: i64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

fn fetch_option_markets_raw() -> Result<Vec<OptionMarket>> {
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct OptionData {
        timezone: String,
        serverTime: i64,
        optionContracts: Vec<Value>,
        optionAssets: Vec<Value>,
        optionSymbols: Vec<OptionMarket>,
    }
    #[derive(Serialize, Deserialize)]
    #[allow(non_snake_case)]
    struct BinanceOptionResponse {
        code: i64,
        msg: String,
        data: OptionData,
    }

    let txt =
        binance_http_get("https://voptions.binance.com/options-api/v1/public/exchange/symbols")?;
    let resp = serde_json::from_str::<BinanceOptionResponse>(&txt)?;
    Ok(resp.data.optionSymbols)
}

pub(super) fn fetch_option_symbols() -> Result<Vec<String>> {
    let symbols = fetch_option_markets_raw()?
        .into_iter()
        .map(|m| m.symbol)
        .collect::<Vec<String>>();
    Ok(symbols)
}

pub(super) fn fetch_option_markets() -> Result<Vec<Market>> {
    let raw_markets = fetch_option_markets_raw()?;
    let markets = raw_markets
        .into_iter()
        .map(|m| {
            let base_currency = m.underlying.strip_suffix(m.quoteAsset.as_str()).unwrap();
            Market {
                exchange: "binance".to_string(),
                market_type: MarketType::Option,
                symbol: m.symbol.clone(),
                pair: format!("{}/{}", base_currency, m.quoteAsset),
                base: base_currency.to_string(),
                quote: m.quoteAsset.clone(),
                base_id: base_currency.to_string(),
                quote_id: m.quoteAsset.clone(),
                active: true,
                margin: true,
                // see https://www.binance.com/en/fee/optionFee
                fees: Fees {
                    maker: m.makerFeeRate,
                    taker: m.takerFeeRate,
                },
                precision: Precision {
                    price: m.priceScale,
                    base: m.quantityScale,
                    quote: None,
                },
                min_quantity: MinQuantity {
                    base: Some(m.minQty),
                    quote: None,
                },
                info: serde_json::to_value(&m)
                    .unwrap()
                    .as_object()
                    .unwrap()
                    .clone(),
            }
        })
        .collect::<Vec<Market>>();
    Ok(markets)
}
