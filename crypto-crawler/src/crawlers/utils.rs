macro_rules! gen_crawl_snapshot {
    ($func_name:ident, $msg_type:expr, $fetch_snapshot:expr) => {
        pub(crate) fn $func_name(
            market_type: MarketType,
            symbols: Option<&[String]>,
            on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
            interval: Option<u64>,
            duration: Option<u64>,
        ) {
            let interval = Duration::from_secs(interval.unwrap_or(60));
            let now = Instant::now();
            loop {
                let loop_start = Instant::now();

                let is_empty = match symbols {
                    Some(list) => {
                        if list.is_empty() {
                            true
                        } else {
                            check_args(market_type, &list);
                            false
                        }
                    }
                    None => true,
                };

                let real_symbols = if is_empty {
                    if std::env::var("https_proxy").is_ok() {
                        // retry retry_count times if there is a https_proxy
                        let retry_count = std::env::var("REST_RETRY_COUNT").unwrap_or("5".to_string()).parse::<i64>().unwrap();
                        let mut symbols = Vec::<String>::new();
                        for i in 0..retry_count {
                            match fetch_symbols(EXCHANGE_NAME, market_type) {
                                Ok(list) => {
                                    symbols = list;
                                    break;
                                }
                                Err(err) => {
                                    if i == retry_count-1 {
                                        error!("The {}th time, {}", i, err);
                                    } else {
                                        info!("The {}th time, {}", i, err);
                                    }
                                }
                            }
                        }
                        symbols
                    } else {
                        match fetch_symbols(EXCHANGE_NAME, market_type) {
                            Ok(symbols) => symbols,
                            Err(err) => {
                                error!("{}", err);
                                Vec::<String>::new()
                            }
                        }
                    }
                } else {
                    symbols.unwrap().iter().cloned().collect::<Vec<String>>()
                };

                for symbol in real_symbols.iter() {
                    let resp = ($fetch_snapshot)(symbol);
                    match resp {
                        Ok(msg) => {
                            let message = Message::new(
                                EXCHANGE_NAME.to_string(),
                                market_type,
                                symbol.to_string(),
                                $msg_type,
                                msg,
                            );
                            (on_msg.lock().unwrap())(message);
                        }
                        Err(err) => error!(
                            "{} {} {}, error: {}",
                            EXCHANGE_NAME, market_type, symbol, err
                        ),
                    }
                }

                if let Some(seconds) = duration {
                    if now.elapsed() > Duration::from_secs(seconds) {
                        break;
                    }
                }
                if loop_start.elapsed() < interval {
                    std::thread::sleep(interval - loop_start.elapsed());
                }
            }
        }
    };
}

macro_rules! gen_crawl_event {
    ($func_name:ident, $struct_name:ident, $msg_type:expr, $crawl_func:ident, $run:expr) => {
        pub(crate) fn $func_name(
            market_type: MarketType,
            symbols: Option<&[String]>,
            on_msg: Arc<Mutex<dyn FnMut(Message) + 'static + Send>>,
            duration: Option<u64>,
        ) -> Option<std::thread::JoinHandle<()>> {
            let is_empty = match symbols {
                Some(list) => {
                    if list.is_empty() {
                        true
                    } else {
                        check_args(market_type, &list);
                        false
                    }
                }
                None => true,
            };

            let real_symbols = if is_empty {
                if std::env::var("https_proxy").is_ok() {
                    // retry retry_count times if there is a https_proxy
                    let retry_count = std::env::var("REST_RETRY_COUNT").unwrap_or("5".to_string()).parse::<i64>().unwrap();
                    let mut symbols = Vec::<String>::new();
                    for i in 0..retry_count {
                        match fetch_symbols(EXCHANGE_NAME, market_type) {
                            Ok(list) => {
                                symbols = list;
                                break;
                            }
                            Err(err) => {
                                if i == retry_count-1 {
                                    error!("The {}th time, {}", i, err);
                                } else {
                                    info!("The {}th time, {}", i, err);
                                }
                            }
                        }
                    }
                    symbols
                } else {
                    match fetch_symbols(EXCHANGE_NAME, market_type) {
                        Ok(symbols) => symbols,
                        Err(err) => {
                            error!("{}", err);
                            Vec::<String>::new()
                        }
                    }
                }
            } else {
                symbols.unwrap().iter().cloned().collect::<Vec<String>>()
            };

            let on_msg_ext = Arc::new(Mutex::new(move |msg: String| {
                let message = Message::new(
                    EXCHANGE_NAME.to_string(),
                    market_type,
                    extract_symbol(&msg),
                    $msg_type,
                    msg.to_string(),
                );
                (on_msg.lock().unwrap())(message);
            }));

            let should_stop = Arc::new(AtomicBool::new(false));
            let ws_client = Arc::new($struct_name::new(on_msg_ext, None));

            if symbols.is_none() {
                let should_stop2 = should_stop.clone();
                let ws_client2 = ws_client.clone();

                std::thread::spawn(move || {
                    while !should_stop2.load(Ordering::Acquire) {
                        let symbols = fetch_symbols(EXCHANGE_NAME, market_type).unwrap();
                        ws_client2.$crawl_func(&symbols);
                        std::thread::sleep(Duration::from_secs(3600));
                    }
                });
            }

            if $run {
                ws_client.$crawl_func(&real_symbols);
                ws_client.run(duration);
                should_stop.store(true, Ordering::Release);
                None
            } else {
                let handle = std::thread::spawn(move || {
                    ws_client.$crawl_func(&real_symbols);
                    ws_client.run(duration);
                    should_stop.store(true, Ordering::Release);
                });
                Some(handle)
            }
        }
    };
}

macro_rules! gen_check_args {
    ($exchange: expr) => {
        use crypto_markets::get_market_types;

        fn check_args(market_type: MarketType, symbols: &[String]) {
            let market_types = get_market_types($exchange);
            if !market_types.contains(&market_type) {
                panic!(
                    "{} does NOT have the {} market type",
                    $exchange, market_type
                );
            }

            let valid_symbols = fetch_symbols($exchange, market_type).unwrap();
            let invalid_symbols: Vec<String> = symbols
                .iter()
                .filter(|symbol| !valid_symbols.contains(symbol))
                .cloned()
                .collect();
            if !invalid_symbols.is_empty() {
                panic!(
                    "Invalid symbols for {} {} market: {}, available trading symbols are {}",
                    $exchange,
                    market_type,
                    invalid_symbols.join(","),
                    valid_symbols.join(",")
                );
            }
        }
    };
}
