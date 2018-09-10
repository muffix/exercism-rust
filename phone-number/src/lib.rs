pub fn number(digits: &str) -> Option<String> {
    let mut digits: Vec<char> = digits.chars().filter(|c| c.is_numeric()).collect();

    if digits.len() == 11 && digits[0] == '1' {
        digits.remove(0);
    }

    match digits.len() {
        10 if digits[0] == '0' || digits[0] == '1' || digits[3] == '0' || digits[3] == '1' => None,
        10 => Some(digits.iter().collect()),
        _ => None,
    }
}
