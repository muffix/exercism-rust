pub fn abbreviate(phrase: &str) -> String {
    phrase
        .chars()
        .map(|c| if c.is_alphabetic() { c } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .map(|word| abbreviate_word(word))
        .collect()
}

fn abbreviate_word(word: &str) -> String {
    let uppercase_chars: String = word.chars().filter(|c| c.is_uppercase()).collect();

    return match uppercase_chars.len() {
        0 => first_char_uppercase(word).to_string(),
        n if n == word.len() => first_char_uppercase(&uppercase_chars).to_string(),
        _ => uppercase_chars,
    };
}

fn first_char_uppercase(word: &str) -> char {
    word.chars().next().unwrap().to_ascii_uppercase()
}
