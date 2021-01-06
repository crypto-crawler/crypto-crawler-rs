macro_rules! impl_contract {
    ($struct_name:ident) => {
        impl $struct_name {
            pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
                Self {
                    _api_key: api_key,
                    _api_secret: api_secret,
                }
            }

            /// Get the most recent trades.
            ///
            /// Equivalent to `/market/history/trade` with `size=2000`
            ///
            /// For example: <https://api.hbdm.com/market/history/trade?symbol=BTC_CQ&size=2000>
            pub fn fetch_trades(symbol: &str) -> Result<String, reqwest::Error> {
                gen_api!(format!("/market/history/trade?symbol={}&size=2000", symbol))
            }

            /// Get the latest L2 orderbook snapshot.
            ///
            /// Top 150 bids and asks (aggregated) are returned.
            ///
            /// `step` controls the aggregaton precision, valid values are
            /// 0, 1, 2, 3, 4, 5, 14, 15
            ///
            /// For example: <https://api.hbdm.com/market/depth?symbol=BTC_CQ&type=step5>
            pub fn fetch_l2_snapshot(symbol: &str, step: i8) -> Result<String, reqwest::Error> {
                gen_api!(format!("/market/depth?symbol={}&type=step{}", symbol, step))
            }
        }
    };
}
