use http::Uri;
use log::*;
use rustls::{ClientConfig, ClientSession, StreamOwned};
use std::{
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    sync::Arc,
    thread,
    time::{self, Duration},
};
use tungstenite::{
    client::{self, AutoStream, IntoClientRequest},
    error::{TlsError, UrlError},
    handshake::{client::Response, HandshakeError},
    stream::{Mode, NoDelay, Stream as StreamSwitcher},
    Error, Result, WebSocket,
};
use webpki::DNSNameRef;

/// The total size of websocket message
pub(crate) const WS_FRAME_SIZE: usize = 4096;

// copied from https://github.com/snapview/tungstenite-rs/blob/master/src/client.rs#L69
fn wrap_stream(stream: TcpStream, domain: &str, mode: Mode) -> Result<AutoStream> {
    match mode {
        Mode::Plain => Ok(StreamSwitcher::Plain(stream)),
        Mode::Tls => {
            let config = {
                let mut config = ClientConfig::new();
                config
                    .root_store
                    .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);

                Arc::new(config)
            };
            let domain = DNSNameRef::try_from_ascii_str(domain).map_err(TlsError::Dns)?;
            let client = ClientSession::new(&config, domain);
            let stream = StreamOwned::new(client, stream);

            Ok(StreamSwitcher::Tls(stream))
        }
    }
}

// copied from https://github.com/snapview/tungstenite-rs/blob/master/src/client.rs#L206
fn connect_to_some(
    addrs: &[SocketAddr],
    uri: &Uri,
    mode: Mode,
    timeout: Option<u64>,
) -> Result<AutoStream> {
    let domain = uri.host().ok_or(Error::Url(UrlError::NoHostName))?;
    for addr in addrs {
        debug!("Trying to contact {} at {}...", uri, addr);
        if let Ok(raw_stream) = TcpStream::connect(addr) {
            if let Some(seconds) = timeout {
                let _ = raw_stream.set_read_timeout(Some(Duration::from_secs(seconds)));
            }
            if let Ok(stream) = wrap_stream(raw_stream, domain, mode) {
                return Ok(stream);
            }
        }
    }
    Err(Error::Url(UrlError::UnableToConnect(uri.to_string())))
}

// Usually ws_stream.read_message() blocks forever,
// with this function, it returns after `timeout` seconds if no data comming in
fn connect_with_timeout(
    url: &str,
    timeout: Option<u64>,
) -> Result<(WebSocket<AutoStream>, Response)> {
    let request = url.into_client_request()?;

    let uri = request.uri();
    let mode = client::uri_mode(uri)?;
    let host = request
        .uri()
        .host()
        .ok_or(Error::Url(UrlError::NoHostName))?;
    let port = uri.port_u16().unwrap_or(match mode {
        Mode::Plain => 80,
        Mode::Tls => 443,
    });
    let addrs = (host, port).to_socket_addrs()?;
    let mut stream = connect_to_some(addrs.as_slice(), request.uri(), mode, timeout)?;
    NoDelay::set_nodelay(&mut stream, true)?;
    tungstenite::client(request, stream).map_err(|e| match e {
        HandshakeError::Failure(f) => f,
        HandshakeError::Interrupted(_) => panic!("Bug: blocking handshake not blocked"),
    })
}

// This function is equivalent to tungstenite::connect(), with an additional benefit that
// it can make read_message() timeout after 5 seconds
pub(super) fn connect_with_retry(url: &str, timeout: Option<u64>) -> WebSocket<AutoStream> {
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
