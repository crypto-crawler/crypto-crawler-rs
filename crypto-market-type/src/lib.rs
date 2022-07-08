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
#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize, Display, Debug, EnumString, PartialEq, Hash, Eq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum MarketType {
    Unknown,
    Spot,
    LinearFuture,
    InverseFuture,
    LinearSwap,
    InverseSwap,

    AmericanOption,
    EuropeanOption,

    QuantoFuture,
    QuantoSwap,

    Move,
    #[serde(rename = "bvol")]
    #[allow(clippy::upper_case_acronyms)]
    BVOL,
}

/// Get market types of a cryptocurrency exchange.
pub fn get_market_types(exchange: &str) -> Vec<MarketType> {
    match exchange {
        "binance" => vec![
            MarketType::Spot,
            MarketType::LinearFuture,
            MarketType::InverseFuture,
            MarketType::LinearSwap,
            MarketType::InverseSwap,
            // MarketType::EuropeanOption, // binance has shutdown option markets.
        ],
        "bitfinex" => vec![MarketType::Spot, MarketType::LinearSwap],
        "bitget" => vec![
            MarketType::Spot,
            MarketType::InverseSwap, // TODO: Bitget's coin-margined swap market is a kind of mixed contract
            MarketType::LinearSwap,
        ],
        "bithumb" => vec![MarketType::Spot],
        // BitMEX only handles Bitcoin. All profit and loss is in Bitcoin
        "bitmex" => vec![
            MarketType::LinearSwap,
            MarketType::InverseSwap,
            MarketType::QuantoSwap,
            MarketType::LinearFuture,
            MarketType::InverseFuture,
            MarketType::QuantoFuture,
        ],
        "bitstamp" => vec![MarketType::Spot],
        "bitz" => vec![
            MarketType::Spot,
            MarketType::InverseSwap,
            MarketType::LinearSwap,
        ],
        "bybit" => vec![
            MarketType::InverseSwap,
            MarketType::LinearSwap,
            MarketType::InverseFuture,
        ],
        "coinbase_pro" => vec![MarketType::Spot],
        // Deribit only accepts Bitcoin as funds to deposit.
        "deribit" => vec![
            MarketType::InverseFuture,
            MarketType::InverseSwap,
            MarketType::EuropeanOption, // inverse
        ],
        "dydx" => vec![MarketType::LinearSwap],
        "ftx" => vec![
            MarketType::Spot,
            MarketType::LinearFuture,
            MarketType::LinearSwap,
            MarketType::Move,
            MarketType::BVOL,
        ],
        "gate" => vec![
            MarketType::Spot,
            MarketType::InverseFuture,
            MarketType::LinearFuture,
            MarketType::InverseSwap,
            MarketType::LinearSwap,
        ],
        "huobi" => vec![
            MarketType::Spot,
            MarketType::InverseFuture,
            MarketType::LinearSwap,
            MarketType::InverseSwap,
            // MarketType::EuropeanOption,
        ],
        "kraken" => vec![
            MarketType::Spot,
            MarketType::InverseFuture,
            MarketType::InverseSwap,
        ],
        "kucoin" => vec![
            MarketType::Spot,
            MarketType::LinearSwap,
            MarketType::InverseSwap,
            MarketType::InverseFuture,
        ],
        "mxc" | "mexc" => vec![
            MarketType::Spot,
            MarketType::LinearSwap,
            MarketType::InverseSwap,
        ],
        "okex" | "okx" => vec![
            MarketType::Spot,
            MarketType::LinearFuture,
            MarketType::InverseFuture,
            MarketType::LinearSwap,
            MarketType::InverseSwap,
            MarketType::EuropeanOption,
        ],
        "zb" => vec![MarketType::Spot, MarketType::LinearSwap],
        "zbg" => vec![
            MarketType::Spot,
            MarketType::InverseSwap,
            MarketType::LinearSwap,
        ],
        _ => panic!("Unknown exchange {}", exchange),
    }
}
