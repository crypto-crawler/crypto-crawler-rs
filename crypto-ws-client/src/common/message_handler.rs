use tokio_tungstenite::tungstenite::Message;

#[derive(Debug)]
pub(crate) enum MiscMessage {
    Normal,             // A normal websocket message which contains a JSON string
    Mutated(String),    // A JSON string mutated by a handler, e.g., bitfinex
    WebSocket(Message), // WebSocket message that needs to be sent to the server
    Pong,               // Pong message from the server
    Reconnect,          // Needs to reconnect
    Other,              // Other messages will be ignored
}

/// Exchange-specific message handler.
pub(crate) trait MessageHandler {
    /// Given a message from the exchange, return a MiscMessage which will be procesed in run().
    fn handle_message(&mut self, msg: &str) -> MiscMessage;
    /// To keep the connection alive, how often should the client send a ping?
    /// None means the client doesn't need to send
    /// ping, instead the server will send ping and the client just needs to reply a pong
    fn get_ping_msg_and_interval(&self) -> Option<(String, u64)>;
}
