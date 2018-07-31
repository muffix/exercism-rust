/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let parsed = validate_input(code);

    match parsed {
        Some(input) => {
            input
                .chars()
                .rev()
                .filter_map(|c| c.to_digit(10))
                .enumerate()
                .map(|(i, d)| if i % 2 == 1 { luhn_double(d) } else { d })
                .sum::<u32>() % 10 == 0
        }
        None => false,
    }
}

fn luhn_double(d: u32) -> u32 {
    if d * 2 > 9 {
        d * 2 - 9
    } else {
        d * 2
    }
}

fn validate_input(code: &str) -> Option<String> {
    let cleaned = code.replace(" ", "");

    if cleaned.parse::<i32>().is_ok() && cleaned.len() > 1 {
        return Some(cleaned);
    }
    None
}
