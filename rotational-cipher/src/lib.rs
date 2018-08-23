pub fn rotate(input: &str, key: i8) -> String {
    input.chars().map(|c| shift_char(c, key)).collect()
}

fn shift_char(c: char, key: i8) -> char {
    match c {
        c if c.is_ascii_uppercase() => ((c as u8 + key as u8 - 'A' as u8) % 26 + 'A' as u8) as char,
        c if c.is_ascii_lowercase() => ((c as u8 + key as u8 - 'a' as u8) % 26 + 'a' as u8) as char,
        c => c,
    }
}
