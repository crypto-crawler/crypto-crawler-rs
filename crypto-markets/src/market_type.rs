use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// Market type.
///
/// * In spot market, cryptocurrencies are traded for immediate delivery, see https://en.wikipedia.org/wiki/Spot_market.
/// * In futures market, delivery is set at a specified time in the future, see https://en.wikipedia.org/wiki/Futures_exchange.
/// * Swap market is a variant of futures market with no expiry date.
///
/// ## Margin
///
/// A market can have margin enabled or disabled.
///
/// * All contract markets are margin enabled, including future, swap and option.
/// * Most spot markets don't have margin enabled, only a few exchanges have spot market with margin enabled.
///
/// ## Linear VS. Inverse
///
/// A market can be inverse or linear.

/// * Linear means USDT-margined, i.e., you can use USDT as collateral
/// * Inverse means coin-margined, i.e., you can use BTC as collateral.
/// * Spot market is always linear.
///
/// **Margin and Inverse are orthogonal.**
#[derive(Copy, Clone, Serialize, Deserialize, Display, Debug, EnumString, PartialEq)]
pub enum MarketType {
    #[serde(rename = "spot")]
    Spot,
    #[serde(rename = "linear_future")]
    LinearFuture,
    #[serde(rename = "inverse_future")]
    InverseFuture,
    #[serde(rename = "linear_swap")]
    LinearSwap,
    #[serde(rename = "inverse_swap")]
    InverseSwap,
    #[serde(rename = "option")]
    Option,

    #[serde(rename = "quanto_future")]
    QuantoFuture,
    #[serde(rename = "quanto_swap")]
    QuantoSwap,
}

/// Get market types of a cryptocurrency exchange.
pub fn get_market_types(exchange: &str) -> Vec<MarketType> {
    match exchange {
        "binance" => vec![
            MarketType::Spot,
            MarketType::InverseFuture,
            MarketType::LinearSwap,
            MarketType::InverseSwap,
            MarketType::Option,
        ],
        "bitfinex" => vec![MarketType::Spot, MarketType::LinearSwap],
        "bitmex" => vec![
            MarketType::InverseSwap,
            MarketType::QuantoSwap,
            MarketType::LinearFuture,
            MarketType::InverseFuture,
            MarketType::QuantoFuture,
        ],
        "bitstamp" => vec![MarketType::Spot],
        "coinbase_pro" => vec![MarketType::Spot],
        "huobi" => vec![
            MarketType::Spot,
            MarketType::InverseFuture,
            MarketType::LinearSwap,
            MarketType::InverseSwap,
            MarketType::Option,
        ],
        "kraken" => vec![MarketType::Spot],
        "mxc" => vec![
            MarketType::Spot,
            MarketType::LinearSwap,
            MarketType::InverseSwap,
        ],
        "okex" => vec![
            MarketType::Spot,
            MarketType::LinearFuture,
            MarketType::InverseFuture,
            MarketType::LinearSwap,
            MarketType::InverseSwap,
            MarketType::Option,
        ],
        _ => panic!("Unknown exchange {}", exchange),
    }
}
