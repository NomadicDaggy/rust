pub fn reply(message: &str) -> &str {
    let has_text = !message.chars().all(char::is_control)
        && (message.chars().any(char::is_alphanumeric)
            || message.chars().any(|c| char::is_ascii_punctuation(&c)));

    let is_question = message.trim_end_matches(char::is_whitespace).chars().last() == Some('?');

    let is_shout =
        (message.to_uppercase() == message) && (message.chars().any(char::is_alphabetic));

    match (has_text, is_question, is_shout) {
        (true, true, true) => "Calm down, I know what I'm doing!",
        (true, false, true) => "Whoa, chill out!",
        (true, false, _) => "Whatever.",
        (_, true, _) => "Sure.",
        (false, false, _) => "Fine. Be that way!",
    }
}
