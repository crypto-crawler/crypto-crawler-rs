macro_rules! gen_crawl_snapshot {
    ($market_type:ident, $symbols:ident, $on_msg:ident, $duration:ident, $msg_type:expr, $fetch_snapshot:expr) => {{
        let mut on_msg_ext = |json: String, symbol: String| {
            let message = Message::new(
                EXCHANGE_NAME.to_string(),
                $market_type,
                symbol,
                $msg_type,
                json,
            );
            ($on_msg)(message);
        };

        let now = Instant::now();
        loop {
            let mut succeeded = false;
            for symbol in $symbols.iter() {
                let resp = ($fetch_snapshot)(symbol);
                match resp {
                    Ok(msg) => {
                        on_msg_ext(msg, symbol.to_string());
                        succeeded = true
                    }
                    Err(err) => error!(
                        "{} {} {}, error: {}",
                        EXCHANGE_NAME, $market_type, symbol, err
                    ),
                }
            }

            if let Some(seconds) = $duration {
                if now.elapsed() > Duration::from_secs(seconds) && succeeded {
                    break;
                }
            }

            std::thread::sleep(Duration::from_secs(crate::SNAPSHOT_INTERVAL));
        }
    }};
}
