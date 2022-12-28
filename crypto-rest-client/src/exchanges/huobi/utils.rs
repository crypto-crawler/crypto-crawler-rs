macro_rules! impl_contract {
    ($struct_name:ident) => {
        impl $struct_name {
            pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
                Self { _api_key: api_key, _api_secret: api_secret }
            }

            /// Get the most recent trades.
            ///
            /// Equivalent to `/market/history/trade` with `size=2000`
            ///
            /// For example: <https://api.hbdm.com/market/history/trade?symbol=BTC_CQ&size=2000>
            pub fn fetch_trades(symbol: &str) -> Result<String> {
                gen_api!(format!("/market/history/trade?symbol={}&size=2000", symbol))
            }
        }
    };
}
