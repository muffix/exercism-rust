pub fn reply(message: &str) -> &str {
    let message = message.trim().to_owned();

    if message.len() == 0 {
        return "Fine. Be that way!";
    }

    let is_question = message.ends_with("?");
    let is_yelled = message.chars().any(|c| c.is_alphabetic()) && message.to_uppercase() == message;

    if is_question && is_yelled {
        return "Calm down, I know what I'm doing!";
    }

    if is_question {
        return "Sure.";
    }

    if is_yelled {
        return "Whoa, chill out!";
    }

    "Whatever."
}
