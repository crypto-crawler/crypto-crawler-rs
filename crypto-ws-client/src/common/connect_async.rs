use std::env;
use futures_util::{SinkExt, StreamExt};
use log::*;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_tungstenite::tungstenite::{Error, Message};

use governor::{Quota, RateLimiter};
use nonzero_ext::*;
use std::num::NonZeroU32;
use fast_socks5::client::{Config, Socks5Stream};
use reqwest::Url;

/// Wraps a websocket client inside an event loop, returns a message_rx to receive messages and
/// a command_tx to send commands to the websocket server.
///
/// To close the websocket connection, send a `Message::Close` message to the command_tx.
///
/// `limit`, max number of uplink messsages, for example, 100 per 10 seconds
pub async fn connect_async(
    url: &str,
    uplink_limit: Option<(NonZeroU32, std::time::Duration)>,
) -> Result<(Receiver<Message>, Sender<Message>), Error> {

    if let Ok(proxy_env) = env::var("https_proxy") {
        let proxy_url = Url::parse(&proxy_env).unwrap();
        let proxy_addr = format!("{}:{}", proxy_url.host_str().unwrap(), proxy_url.port_or_known_default().unwrap());
        match proxy_url.scheme().to_lowercase().as_str() {
            "socks5" => connect_async_with_socks5_proxy(url, &proxy_addr, uplink_limit).await,
            _ => panic!("proxy scheme not implement")
        }

    } else {
        connect_async_direct(url, uplink_limit).await
    }
}

pub async fn connect_async_direct(
    url: &str,
    uplink_limit: Option<(NonZeroU32, std::time::Duration)>,
) -> Result<(Receiver<Message>, Sender<Message>), Error> {
    let (command_tx, mut command_rx) = tokio::sync::mpsc::channel::<Message>(1);
    let (message_tx, message_rx) = tokio::sync::mpsc::channel::<Message>(32);

    let ret = tokio_tungstenite::connect_async(url).await;
    if let Err(e) = ret {
        return Err(e);
    }
    let (ws_stream, _) = ret.unwrap();
    let (mut write, mut read) = ws_stream.split();

    let limiter = if let Some((max_burst, duration)) = uplink_limit {
        let quota = Quota::with_period(duration).unwrap().allow_burst(max_burst);
        RateLimiter::direct(quota)
    } else {
        RateLimiter::direct(Quota::per_second(nonzero!(u32::max_value())))
    };

    tokio::task::spawn(async move {
        loop {
            tokio::select! {
              command = command_rx.recv() => {
                match command {
                  Some(command) => {
                    match command {
                      Message::Close(_) => {
                        break; // close the connection and break the loop
                      }
                      _ => {
                        limiter.until_ready().await;
                        if let Err(err) =write.send(command).await {
                          error!("{:#?}", err);
                        }
                      }
                    }
                  }
                  None => {
                    debug!("command_rx closed");
                    break;
                  }
                }
              }
              msg = read.next() => match msg {
                Some(Ok(msg)) => {
                  let _= message_tx.send(msg).await;
                }
                Some(Err(err)) => {
                  error!("{}", err);
                  break;
                }
                None => {
                  debug!("message_tx closed");
                  break;
                }
              }
            };
        }
        write.send(Message::Close(None)).await;
    });

    Ok((message_rx, command_tx))
}

pub async fn connect_async_with_socks5_proxy(
    url: &str,
    proxy_addr: &str,
    uplink_limit: Option<(NonZeroU32, std::time::Duration)>,
) -> Result<(Receiver<Message>, Sender<Message>), Error> {
    let (command_tx, mut command_rx) = tokio::sync::mpsc::channel::<Message>(1);
    let (message_tx, message_rx) = tokio::sync::mpsc::channel::<Message>(32);
    // replace with socks5 stream
    let connect_url = Url::parse(url).unwrap();
    let proxy_stream = Socks5Stream::connect(
        proxy_addr.to_string(),
        connect_url.host_str().unwrap().to_string(),
        connect_url.port().unwrap(),
        Config::default()
    ).await.unwrap();
    let ret = tokio_tungstenite::client_async_tls(
        connect_url,
        proxy_stream
    ).await;
    // replaced
    // let ret = tokio_tungstenite::connect_async(url).await;
    if let Err(e) = ret {
        return Err(e);
    }
    let (ws_stream, _) = ret.unwrap();
    let (mut write, mut read) = ws_stream.split();

    let limiter = if let Some((max_burst, duration)) = uplink_limit {
        let quota = Quota::with_period(duration).unwrap().allow_burst(max_burst);
        RateLimiter::direct(quota)
    } else {
        RateLimiter::direct(Quota::per_second(nonzero!(u32::max_value())))
    };

    tokio::task::spawn(async move {
        loop {
            tokio::select! {
              command = command_rx.recv() => {
                match command {
                  Some(command) => {
                    match command {
                      Message::Close(_) => {
                        break; // close the connection and break the loop
                      }
                      _ => {
                        limiter.until_ready().await;
                        if let Err(err) =write.send(command).await {
                          error!("{:#?}", err);
                        }
                      }
                    }
                  }
                  None => {
                    debug!("command_rx closed");
                    break;
                  }
                }
              }
              msg = read.next() => match msg {
                Some(Ok(msg)) => {
                  let _= message_tx.send(msg).await;
                }
                Some(Err(err)) => {
                  error!("{}", err);
                  break;
                }
                None => {
                  debug!("message_tx closed");
                  break;
                }
              }
            };
        }
        write.send(Message::Close(None)).await;
    });

    Ok((message_rx, command_tx))
}

#[cfg(test)]
mod tests{
    use reqwest::Url;

    #[test]
    fn test_url(){
        let endpoint = Url::parse("socks5://127.0.0.1:10808").unwrap();
        let proxy_addr = format!("{}:{}", endpoint.host_str().unwrap(), endpoint.port_or_known_default().unwrap());
        eprintln!("{}", proxy_addr);
        let endpoint = Url::parse("http://127.0.0.1:10809").unwrap();
        let proxy_addr = format!("{}:{}", endpoint.host_str().unwrap(), endpoint.port_or_known_default().unwrap());
        eprintln!("{}", proxy_addr);
        let endpoint = Url::parse("https://127.0.0.1:10809").unwrap();
        let proxy_addr = format!("{}:{}", endpoint.host_str().unwrap(), endpoint.port_or_known_default().unwrap());
        eprintln!("{}", proxy_addr);
        let endpoint = Url::parse("https://example.com").unwrap();
        let proxy_addr = format!("{}:{}", endpoint.host_str().unwrap(), endpoint.port_or_known_default().unwrap());
        eprintln!("{}", proxy_addr);
        let endpoint = Url::parse("https://example.com:8443").unwrap();
        let proxy_addr = format!("{}:{}", endpoint.host_str().unwrap(), endpoint.port_or_known_default().unwrap());
        eprintln!("{}", proxy_addr);
    }
}