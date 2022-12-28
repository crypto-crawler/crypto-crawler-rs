/// Ensure that length of a websocket message does not exceed the max size or
/// the number of topics does not exceed the threshold.
pub(crate) fn ensure_frame_size(
    topics: &[(String, String)],
    subscribe: bool,
    topics_to_command: fn(&[(String, String)], bool) -> String,
    max_bytes: usize,
    max_topics_per_command: Option<usize>,
) -> Vec<String> {
    let mut all_commands: Vec<String> = Vec::new();

    let mut begin = 0;
    while begin < topics.len() {
        for end in (begin + 1)..(topics.len() + 1) {
            let num_subscriptions = end - begin;
            let chunk = &topics[begin..end];
            let command = topics_to_command(chunk, subscribe);
            if end == topics.len() {
                all_commands.push(command);
                begin = end;
            } else if num_subscriptions >= max_topics_per_command.unwrap_or(usize::MAX) {
                all_commands.push(command);
                begin = end;
                break;
            } else {
                let chunk = &topics[begin..end + 1];
                let command_next = topics_to_command(chunk, subscribe);
                if command_next.len() > max_bytes {
                    all_commands.push(command);
                    begin = end;
                    break;
                }
            };
        }
    }

    all_commands
}

pub(crate) fn topic_to_raw_channel(topic: &(String, String)) -> String {
    topic.0.replace("SYMBOL", topic.1.as_str())
}
