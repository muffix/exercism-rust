/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn_chars = isbn.chars().filter_map(|c| parse(c)).collect::<Vec<u32>>();

    let isbn_length = isbn_chars.len();

    if isbn_length != 10 {
        return false;
    }

    if isbn_chars[..(isbn_length - 1)].iter().any(|d| *d == 10) {
        return false;
    }

    isbn_chars
        .iter()
        .enumerate()
        .map(|(i, d)| *d * (10 - i as u32))
        .fold(0, |acc, s| acc + s) % 11 == 0
}

fn parse(c: char) -> Option<u32> {
    match c {
        _ if c.is_numeric() => Some(c.to_digit(10).unwrap()),
        'X' => Some(10),
        _ => None,
    }
}
