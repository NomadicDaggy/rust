pub fn reply(message: &str) -> &str {
    let has_text = !message.chars().all(char::is_control) && 
        (
            message.chars().any(char::is_alphanumeric) || 
            message.chars().any(|c| char::is_ascii_punctuation(&c))
        );

    let is_question = message.trim_end_matches(char::is_whitespace).chars().last() == Some('?');

    let is_shout =
        (message.to_uppercase() == message) && (message.chars().any(char::is_alphabetic));

    if has_text {
        if is_question {
            if is_shout {
                return "Calm down, I know what I'm doing!";
            } else {
                return "Sure.";
            }
        } else {
            if is_shout {
                return "Whoa, chill out!";
            } else {
                return "Whatever.";
            }
        }
    } else {
        if is_question {
            return "Sure.";
        }
    }

    "Fine. Be that way!"
}
