//! A versatile websocket client that supports many cryptocurrency exchanges.
//!
//! ## Example
//!
//! ```
//! use crypto_ws_client::{BinanceSpotWSClient, WSClient};
//!
//! let (tx, rx) = std::sync::mpsc::channel();
//! let thread = std::thread::spawn(move || {
//!     for msg in rx {
//!         println!("{}", msg);
//!     }
//! });
//!
//! let mut ws_client = BinanceSpotWSClient::new(tx, None);
//! let channels = vec!["btcusdt@aggTrade".to_string(), "btcusdt@depth".to_string(),];
//! ws_client.subscribe(&channels);
//! ws_client.run(Some(2)); // run for 2 seconds
//! ws_client.close();
//! drop(ws_client);
//! thread.join().unwrap();
//! ```
//! ## High Level APIs
//!
//! The following APIs are high-level APIs with ease of use:
//!
//! * `subscribe_trade(&mut self, pairs: &[String])`
//! * `subscribe_bbo(&mut self, pairs: &[String])`
//! * `subscribe_orderbook(&mut self, pairs: &[String])`
//! * `subscribe_ticker(&mut self, pairs: &[String])`
//! * `subscribe_candlestick(&mut self, pairs: &[String], interval: u32)`
//!
//! They are easier to use and cover mostly used scenarios.
//!
//! ## Low Level APIs
//!
//! Sometimes high-level APIs can NOT meet our needs, this package provides two
//! low-level APIs:
//!
//! * `subscribe(&mut self, raw_channels: &[String])`
//! * `unsubscribe(&mut self, raw_channels: &[String])`
//!
//! A `raw_channel` can be:
//!
//! * A plain string, supported by Binance, BitMEX, Bitstamp, Huobi, OKEx.
//! For example, Binance `btcusdt@aggTrade`, BitMEX `trade:XBTUSD,
//! `instrument`, Bitstamp `live_trades_btcusd`,
//! Huobi `market.btcusdt.trade.detail`, `market.overview`,
//! OKEx `spot/trade:BTC-USDT`.
//! * channel:pair. For exchanges not supporting plain string channels,
//! this library will split this kind of `raw_channel` by `:`, then
//! compose a JSON string. For example, Bitfinex `trades:tBTCUST`,
//! CoinbasePro `matches:BTC-USD`, Kraken `trade:XBT/USD`
//! * A JSON string, supported by all exchanges. If a `raw_channel` starts
//! with `{`, which means it is the final JSON string, thus it will be
//! sent out directly without parsing.
//!
//! ## OrderBook Data Categories
//!
//! Each orderbook has three properties: `aggregation`, `frequency` and `depth`.
//!
//! |                      | Aggregated        | Non-Aggregated |
//! | -------------------- | ----------------- | -------------- |
//! | Updated per Tick     | Inremental Level2 | Level3         |
//! | Updated per Interval | Snapshot Level2   | Not Usefull    |
//!
//! * Level1 data is non-aggregated, updated per tick, top 1 bid & ask from the original orderbook.
//! * Level2 data is aggregated by price level, updated per tick.
//! * Level3 data is the original orderbook, which is not aggregated.

mod clients;

pub use clients::binance::*;
pub use clients::binance_option::*;
// pub use clients::bitfinex::*;
pub use clients::bitfinex::*;
pub use clients::bitget::*;
pub use clients::bithumb::*;
pub use clients::bitmex::*;
pub use clients::bitstamp::*;
pub use clients::bitz::*;
pub use clients::bybit::*;
pub use clients::coinbase_pro::*;
pub use clients::deribit::*;
pub use clients::dydx::*;
pub use clients::ftx::*;
pub use clients::gate::*;
pub use clients::huobi::*;
pub use clients::kraken::*;
pub use clients::kraken_futures::*;
pub use clients::kucoin::*;
pub use clients::mxc::*;
pub use clients::okex::*;
pub use clients::zbg::*;

/// The public interface of every WebSocket client.
pub trait WSClient {
    /// Subscribes to trade channels.
    ///
    /// A trade channel sends tick-by-tick trade data,  which is the complete
    /// copy of a market's trading information.
    ///
    /// Each exchange has its own pair formats, for example:
    ///
    /// * BitMEX `XBTUSD`, `XBTM21`
    /// * Binance `btcusdt`, `btcusd_perp`
    /// * OKEx `BTC-USDT`
    fn subscribe_trade(&self, pairs: &[String]);

    /// Subscribes to BBO(best bid & offer) channels.
    ///
    /// BBO represents best bid and offer, which is also refered to as level1
    /// data. It is the top 1 bid and ask from the orginal orderbook, so BBO
    /// is updated per tick and non-aggregated.
    ///
    /// Not all exchanges have the BBO channel, calling this function with
    /// these exchanges will panic.
    ///
    /// * Binance, BitMEX, Huobi and Kraken have BBO directly.
    /// * Bitfinex uses `book` channel with `len=1` and `prec="R0"` to get BBO data.
    fn subscribe_bbo(&self, pairs: &[String]);

    /// Subscribes to incremental level2 orderbook channels.
    ///
    /// An incremental level2 orderbook channel sends a snapshot followed by
    /// tick-by-tick updates.
    ///
    /// Level2 orderbook is the raw orderbook(Level3) aggregated by price level, it is
    /// also refered to as "market by price level" data.
    ///
    /// This function subscribes to exchange specific channels as the following:
    ///
    /// * Binance `depth`
    /// * Bitfinex `book` channel with `prec=P0`, `frec=F0` and `len=25`
    /// * BitMEX `orderBookL2_25`
    /// * Bitstamp `diff_order_book`, top 100
    /// * CoinbasePro `level2`
    /// * Huobi `depth.size_20.high_freq` with `data_type=incremental` for contracts, `mbp.20` for Spot
    /// * Kraken `book` with `depth=25`
    /// * MXC `depth` for Swap, `symbol` for Spot
    /// * OKEx `depth_l2_tbt`, top 100
    fn subscribe_orderbook(&self, pairs: &[String]);

    /// Subscribes to level2 orderbook snapshot channels.
    ///
    /// A level2 orderbook snapshot channel sends a complete snapshot every interval.
    ///
    /// This function subscribes to exchange specific channels as the following:
    ///
    /// * Binance `depth5`, every 1000ms
    /// * Bitfinex has no snapshot channel
    /// * BitMEX `orderBook10`, top 10, every tick
    /// * Bitstamp `order_book`, top 10, every 100ms
    /// * CoinbasePro has no snapshot channel
    /// * Huobi `depth.step1` and `depth.step7`, top 20, every 1s
    /// * Kraken has no snapshot channel
    /// * MXC `depth.full` for Swap, top 20, every 100ms; `get.depth` for Spot, full, every 26s
    /// * OKEx `depth5`, top 5, every 100ms
    fn subscribe_orderbook_topk(&self, pairs: &[String]);

    /// Subscribes to level3 orderebook channels.
    ///
    /// **Only bitfinex, bitstamp, coinbase_pro and kucoin have level3 orderbook channels.**
    ///
    /// The level3 orderbook is the orginal orderbook of an exchange, it is
    /// non-aggregated by price level and updated tick-by-tick.
    fn subscribe_l3_orderbook(&self, symbols: &[String]);

    /// Subscribes to ticker channels.
    ///
    /// A ticker channel pushes realtime 24hr rolling window ticker messages,
    /// which contains OHLCV information.
    ///
    /// Not all exchanges have the ticker channel, for example, BitMEX,
    /// Bitstamp, MXC Spot, etc.
    fn subscribe_ticker(&self, pairs: &[String]);

    /// Subscribes to candlestick channels.
    ///
    /// The candlestick channel sends OHLCV messages at interval.
    ///
    /// `symbol_interval_list` is a list of symbols and intervals of candlesticks in seconds.
    ///
    /// Not all exchanges have candlestick channels, for example, Bitstamp
    /// and CoinbasePro.
    fn subscribe_candlestick(&self, symbol_interval_list: &[(String, usize)]);

    /// Subscribes to raw channels, lower level API.
    ///
    /// A `raw_channel` can be:
    ///
    /// * A plain string, supported by Binance, BitMEX, Bitstamp, Bybit, Deribit, Huobi, OKEx, ZBG.
    /// For example, Binance `btcusdt@aggTrade`, BitMEX `trade:XBTUSD,
    /// `instrument`, Bitstamp `live_trades_btcusd`,
    /// Huobi `market.btcusdt.trade.detail`, `market.overview`,
    /// OKEx `spot/trade:BTC-USDT`.
    /// * channel:pair. For exchanges not supporting plain string channels,
    /// this library will split this kind of `raw_channel` by `:`, then
    /// compose a JSON string. For example, Bitfinex `trades:tBTCUST`,
    /// CoinbasePro `matches:BTC-USD`, Kraken `trade:XBT/USD`
    /// * A JSON string, supported by all exchanges. If a `raw_channel` starts
    /// with `{`, which means it is the final JSON string, thus it will be
    /// sent out directly without parsing.
    fn subscribe(&self, raw_channels: &[String]);

    /// Unsubscribes from raw channels, lower level API.
    fn unsubscribe(&self, raw_channels: &[String]);

    /// Starts the infinite loop until time is up or the server closes the connection.
    ///
    /// # Arguments
    ///
    /// * `duration` - How many seconds to run, None means infinite.
    fn run(&self, duration: Option<u64>);

    /// Breaks the loop and closes the connection.
    fn close(&self);
}
