//! A versatile websocket client that supports many cryptocurrency exchanges.
//!
//! ## Example
//!
//! ```
//! use crypto_ws_client::{BinanceSpotWSClient, WSClient};
//!
//! fn main() {
//!     let mut ws_client = BinanceSpotWSClient::init(|msg| println!("{}", msg), None);
//!     let channels = vec!["btcusdt@aggTrade".to_string(), "btcusdt@depth".to_string(),];
//!     ws_client.subscribe(&channels);
//!     ws_client.run(Some(5)); // run for 5 seconds
//!     ws_client.close();
//! }
//! ```

mod clients;

pub use clients::binance::*;
pub use clients::huobi::*;
pub use clients::okex::*;

/// The public interface of every WebSocket client.
pub trait WSClient {
    /// Create a new client.
    ///
    /// # Arguments
    ///
    /// * `on_msg` - The message handler
    /// * `url` - Optional server url, usually you don't need specify it
    fn init(on_msg: fn(String), url: Option<&str>) -> Self;

    /// Subscribe channels.
    fn subscribe(&mut self, channels: &[String]);

    /// Unsubscribe channels.
    fn unsubscribe(&mut self, channels: &[String]);

    /// Start the infinite loop until the server closes the connection.
    ///
    /// # Arguments
    ///
    /// * `duration` - How many seconds to run, None means infinite.
    fn run(&mut self, duration: Option<u64>);

    /// Close the client.
    fn close(&mut self);
}
