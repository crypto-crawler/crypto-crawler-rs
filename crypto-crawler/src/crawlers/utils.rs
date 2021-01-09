macro_rules! gen_crawl_snapshot {
    ($func_name:ident, $market_type:ident, $symbols:ident, $on_msg:ident, $duration:ident, $msg_type:expr, $fetch_snapshot:expr) => {
        pub(crate) fn $func_name<'a>(
            $market_type: MarketType,
            $symbols: &[String],
            $on_msg: Rc<RefCell<dyn FnMut(Message) + 'a>>,
            $duration: Option<u64>,
        ) {
            check_args($market_type, $symbols);
            let on_msg_ext = |json: String, symbol: String| {
                let message = Message::new(
                    EXCHANGE_NAME.to_string(),
                    $market_type,
                    symbol,
                    $msg_type,
                    json,
                );
                ($on_msg.borrow_mut())(message);
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
        }
    };
}

macro_rules! gen_crawl_event {
    ($func_name:ident, $market_type:ident, $symbols:ident, $on_msg:ident, $duration:ident, $struct_name:ident, $msg_type:expr, $crawl_func:ident) => {
        pub(crate) fn $func_name<'a>(
            $market_type: MarketType,
            $symbols: &[String],
            $on_msg: Rc<RefCell<dyn FnMut(Message) + 'a>>,
            $duration: Option<u64>,
        ) {
            check_args($market_type, $symbols);
            let on_msg_ext = |msg: String| {
                let message = Message::new(
                    EXCHANGE_NAME.to_string(),
                    $market_type,
                    extract_symbol(&msg),
                    $msg_type,
                    msg.to_string(),
                );
                ($on_msg.borrow_mut())(message);
            };
            let mut ws_client = $struct_name::new(Rc::new(RefCell::new(on_msg_ext)), None);
            ws_client.$crawl_func($symbols);
            ws_client.run($duration);
        }
    };
}
