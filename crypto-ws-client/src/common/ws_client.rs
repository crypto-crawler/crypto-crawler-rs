use async_trait::async_trait;

/// The public interface of every WebSocket client.
#[async_trait]
pub trait WSClient {
    /// Subscribes to trade channels.
    ///
    /// A trade channel sends tick-by-tick trade data,  which is the complete
    /// copy of a market's trading information.
    ///
    /// Each exchange has its own symbol formats, for example:
    ///
    /// * BitMEX `XBTUSD`, `XBTM21`
    /// * Binance `btcusdt`, `btcusd_perp`
    /// * OKEx `BTC-USDT`
    async fn subscribe_trade(&self, symbols: &[String]);

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
    /// * Bitfinex uses `book` channel with `len=1` and `prec="R0"` to get BBO
    ///   data.
    async fn subscribe_bbo(&self, symbols: &[String]);

    /// Subscribes to incremental level2 orderbook channels.
    ///
    /// An incremental level2 orderbook channel sends a snapshot followed by
    /// tick-by-tick updates.
    ///
    /// Level2 orderbook is the raw orderbook(Level3) aggregated by price level,
    /// it is also refered to as "market by price level" data.
    ///
    /// This function subscribes to exchange specific channels as the following:
    ///
    /// * Binance `depth`
    /// * Bitfinex `book` channel with `prec=P0`, `frec=F0` and `len=25`
    /// * BitMEX `orderBookL2_25`
    /// * Bitstamp `diff_order_book`, top 100
    /// * CoinbasePro `level2`
    /// * Huobi `depth.size_20.high_freq` with `data_type=incremental` for
    ///   contracts, `mbp.20` for Spot
    /// * Kraken `book` with `depth=25`
    /// * MEXC `depth` for Swap, `symbol` for Spot
    /// * OKEx `depth_l2_tbt`, top 100
    async fn subscribe_orderbook(&self, symbols: &[String]);

    /// Subscribes to level2 orderbook snapshot channels.
    ///
    /// A level2 orderbook snapshot channel sends a complete snapshot every
    /// interval.
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
    /// * MEXC `depth.full` for Swap, top 20, every 100ms; `get.depth` for Spot,
    ///   full, every 26s
    /// * OKEx `depth5`, top 5, every 100ms
    async fn subscribe_orderbook_topk(&self, symbols: &[String]);

    /// Subscribes to level3 orderebook channels.
    ///
    /// **Only bitfinex, bitstamp, coinbase_pro and kucoin have level3 orderbook
    /// channels.**
    ///
    /// The level3 orderbook is the orginal orderbook of an exchange, it is
    /// non-aggregated by price level and updated tick-by-tick.
    async fn subscribe_l3_orderbook(&self, symbols: &[String]);

    /// Subscribes to ticker channels.
    ///
    /// A ticker channel pushes realtime 24hr rolling window ticker messages,
    /// which contains OHLCV information.
    ///
    /// Not all exchanges have the ticker channel, for example, BitMEX,
    /// Bitstamp, MEXC Spot, etc.
    async fn subscribe_ticker(&self, symbols: &[String]);

    /// Subscribes to candlestick channels.
    ///
    /// The candlestick channel sends OHLCV messages at interval.
    ///
    /// `symbol_interval_list` is a list of symbols and intervals of
    /// candlesticks in seconds.
    ///
    /// Not all exchanges have candlestick channels, for example, Bitstamp
    /// and CoinbasePro.
    async fn subscribe_candlestick(&self, symbol_interval_list: &[(String, usize)]);

    /// Subscribe to multiple topics.
    ///
    /// topic = channel + symbol, a topic will be converted to an
    /// exchange-specific channel(called `raw channel`).
    ///
    /// The channel string supports `SYMBOL` as a placeholder, for example,
    /// `("market.SYMBOL.trade.detail", "btcusdt")` will be converted to a raw
    /// channel `"market.btcusdt.trade.detail"`.
    ///
    /// Examples:
    ///
    /// * Binance: `vec![("aggTrade".to_string(),
    ///   "BTCUSDT".to_string()),("ticker".to_string(), "BTCUSDT".to_string())]`
    /// * Deribit: `vec![(trades.SYMBOL.100ms".to_string(),"BTC-PERPETUAL".
    ///   to_string()),(trades.SYMBOL.100ms".to_string(),"ETH-PERPETUAL".
    ///   to_string())]`
    /// * Huobi: `vec![("trade.detail".to_string(),
    ///   "btcusdt".to_string()),("trade.detail".to_string(),
    ///   "ethusdt".to_string())]`
    /// * OKX: `vec![("trades".to_string(),
    ///   "BTC-USDT".to_string()),("trades".to_string(),
    ///   "ETH-USDT".to_string())]`
    async fn subscribe(&self, topics: &[(String, String)]);

    /// Unsubscribes multiple topics.
    ///
    /// topic = channel + symbol
    async fn unsubscribe(&self, topics: &[(String, String)]);

    /// Send raw JSON commands.
    ///
    /// This is a low-level API for advanced users only.
    async fn send(&self, commands: &[String]);

    /// Starts the infinite event loop.
    async fn run(&self);

    /// Close the connection and break the loop in Run().
    async fn close(&self);
}
