pub fn reply(message: &str) -> &str {
    let question = message.trim().chars().last().unwrap() == '?';
    let yell = message.to_uppercase() == message;

    if question { 
        if yell {
            return "Calm down, I know what I'm doing!";
        } else {
            return "Sure.";
        }
    } else {
        if yell {
            return "Whoa, chill out!";
        }
    }
    "Whatever."
}
