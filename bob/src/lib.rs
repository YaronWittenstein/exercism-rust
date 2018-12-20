pub fn reply(message: &str) -> &str {
    let message = message.trim();

    if is_silence(message) {
        "Fine. Be that way!"
    } else if is_shout(message) && is_question(message) {
        "Calm down, I know what I'm doing!"
    } else if is_shout(message) {
        "Whoa, chill out!"
    } else if is_question(message) {
        "Sure."
    } else {
        "Whatever."
    }
}

fn is_shout(message: &str) -> bool {
    has_letters(message) && message.to_uppercase() == message
}

fn has_letters(message: &str) -> bool {
    message.chars().any(|ch| ch.is_ascii_alphabetic())
}

fn is_question(message: &str) -> bool {
    message.ends_with('?')
}

fn is_silence(message: &str) -> bool {
    message.chars().all(|ch| ch.is_whitespace())
}