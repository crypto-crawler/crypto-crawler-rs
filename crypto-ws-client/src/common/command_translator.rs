/// Translate to exchange-specific websocket subscribe/unsubscribe commands.
///
/// topic = channel + symbol
///
/// A command is a JSON string which can be aceepted by the websocket server,
/// and every exchange has its own format.
pub(crate) trait CommandTranslator {
    fn translate_to_commands(&self, subscribe: bool, topics: &[(String, String)]) -> Vec<String>;
    fn translate_to_candlestick_commands(
        &self,
        subscribe: bool,
        symbol_interval_list: &[(String, usize)],
    ) -> Vec<String>;
}
