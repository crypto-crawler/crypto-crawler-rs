use std::future::Future;

use crate::msg::*;
use crypto_markets::MarketType;

mod crawlers;
mod msg;

pub async fn crawl<Fut>(
    exchange: &str,
    market_type: MarketType,
    channel_types: &[ChannelType],
    pairs: &[&str],
    msg_callback: impl Fn(Msg) -> Fut,
) -> ()
where
    Fut: Future<Output = ()>,
{
    return;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
