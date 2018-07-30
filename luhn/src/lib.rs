/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut checksum = 0;
    let mut i = 0;

    for c in code.chars().rev() {
        if c == ' ' {
            continue;
        }

        if !c.is_digit(10) {
            return false;
        }

        let digit = c.to_digit(10).unwrap() * ((i % 2) + 1);

        checksum += digit;
        if digit > 9 {
            checksum -= 9
        }

        i += 1;
    }

    i > 1 && checksum % 10 == 0
}
