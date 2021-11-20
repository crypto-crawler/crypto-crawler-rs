use http::Uri;
use log::*;
use std::{
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    thread,
    time::{self, Duration},
};
use tungstenite::{
    client::{uri_mode, IntoClientRequest},
    client_tls,
    error::UrlError,
    handshake::{client::Response, HandshakeError},
    stream::{MaybeTlsStream, Mode, NoDelay},
    Error, Result, WebSocket,
};

// copied from https://github.com/snapview/tungstenite-rs/blob/master/src/client.rs#L122
fn connect_to_some(addrs: &[SocketAddr], uri: &Uri, timeout: Option<u64>) -> Result<TcpStream> {
    for addr in addrs {
        debug!("Trying to contact {} at {}...", uri, addr);
        if let Ok(stream) = TcpStream::connect(addr) {
            if let Some(seconds) = timeout {
                let _ = stream.set_read_timeout(Some(Duration::from_secs(seconds)));
            }
            return Ok(stream);
        }
    }
    Err(Error::Url(UrlError::UnableToConnect(uri.to_string())))
}

// Usually ws_stream.read_message() blocks forever,
// with this function, it returns after `timeout` seconds if no data comming in
fn connect_with_timeout(
    url: &str,
    timeout: Option<u64>,
) -> Result<(WebSocket<MaybeTlsStream<TcpStream>>, Response)> {
    let request = url.into_client_request()?;

    let uri = request.uri();
    let mode = uri_mode(uri)?;
    let host = request
        .uri()
        .host()
        .ok_or(Error::Url(UrlError::NoHostName))?;
    let port = uri.port_u16().unwrap_or(match mode {
        Mode::Plain => 80,
        Mode::Tls => 443,
    });
    let addrs = (host, port).to_socket_addrs()?;
    let mut stream = connect_to_some(addrs.as_slice(), request.uri(), timeout)?;
    NoDelay::set_nodelay(&mut stream, true)?;
    let client = client_tls(request, stream);

    client.map_err(|e| match e {
        HandshakeError::Failure(f) => f,
        HandshakeError::Interrupted(_) => panic!("Bug: blocking handshake not blocked"),
    })
}

// This function is equivalent to tungstenite::connect(), with an additional benefit that
// it can make read_message() timeout after 5 seconds
pub(super) fn connect_with_retry(
    url: &str,
    timeout: Option<u64>,
) -> WebSocket<MaybeTlsStream<TcpStream>> {
    let max_count = 5;
    let mut backoff_factor = 1;
    let backoff_duration = time::Duration::from_secs(if url.contains("bitmex") { 16 } else { 4 });
    let mut error_msg: String = String::new();
    for i in 0..max_count {
        let res = connect_with_timeout(url, timeout);
        match res {
            Ok((ws_stream, _)) => return ws_stream,
            Err(err) => {
                error_msg = err.to_string();
                if error_msg.contains("429") {
                    backoff_factor += 1;
                } else {
                    backoff_factor *= 2;
                }
                warn!(
                    "Failed connecting to {} the {}th time, error: {}",
                    url, i, err
                );
                thread::sleep(backoff_duration * backoff_factor);
            }
        }
    }

    panic!("Error connecting to {}, error: {}, aborted", url, error_msg);
}

pub(super) const CHANNEL_PAIR_DELIMITER: char = ':';

/// Ensure that length of a websocket message does not exceed the max size or the number of topics does not exceed the threshold.
pub(crate) fn ensure_frame_size(
    channels: &[String],
    subscribe: bool,
    topics_to_command: fn(&[String], bool) -> String,
    max_bytes: usize,
    max_topics_per_command: Option<usize>,
) -> Vec<String> {
    let raw_channels: Vec<String> = channels
        .iter()
        .filter(|ch| !ch.starts_with('{'))
        .cloned()
        .collect();
    let mut all_commands: Vec<String> = channels
        .iter()
        .filter(|ch| ch.starts_with('{'))
        .cloned()
        .collect();

    if !raw_channels.is_empty() {
        let mut begin = 0;
        while begin < raw_channels.len() {
            for end in (begin + 1)..(raw_channels.len() + 1) {
                let num_subscriptions = end - begin;
                let chunk = &raw_channels[begin..end];
                let command = topics_to_command(chunk, subscribe);
                if end == raw_channels.len() {
                    all_commands.push(command);
                    begin = end;
                } else if num_subscriptions >= max_topics_per_command.unwrap_or(usize::MAX) {
                    all_commands.push(command);
                    begin = end;
                    break;
                } else {
                    let chunk = &raw_channels[begin..end + 1];
                    let command_next = topics_to_command(chunk, subscribe);
                    if command_next.len() > max_bytes {
                        all_commands.push(command);
                        begin = end;
                        break;
                    }
                };
            }
        }
    };

    all_commands
}
