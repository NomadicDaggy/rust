pub fn reply(message: &str) -> &str {
    let m = message.trim();

    let has_text = !m.is_empty();
    let is_question = m.ends_with('?');
    let is_shout = (m.to_uppercase() == m) && (m.contains(char::is_alphabetic));

    match (has_text, is_question, is_shout) {
        (true, true, true) => "Calm down, I know what I'm doing!",
        (true, false, true) => "Whoa, chill out!",
        (true, false, _) => "Whatever.",
        (_, true, _) => "Sure.",
        (false, false, _) => "Fine. Be that way!",
    }
}
