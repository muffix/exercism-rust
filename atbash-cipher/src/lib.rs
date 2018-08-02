/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() && c.is_ascii())
        .map(convert)
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|ch| ch.iter().cloned().collect())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(convert)
        .collect()
}

fn convert(c: char) -> char {
    if c.is_numeric() {
        c
    } else {
        ('z' as u8 + 'a' as u8 - c as u8) as char
    }
}
