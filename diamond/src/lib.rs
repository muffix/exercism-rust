pub fn get_diamond(c: char) -> Vec<String> {
    let mut rows = Vec::new();

    let mut current = 'A';

    while current <= c {
        let skip = " ".repeat(c as usize - current as usize);

        if current == 'A' {
            rows.push(format!("{}{}{}", skip, current, skip));
        } else {
            let middle = " ".repeat(2 * (current as usize - 'A' as usize) - 1);
            rows.push(format!("{}{}{}{}{}", skip, current, middle, current, skip));
        }

        current = (current as u8 + 1) as char;
    }

    rows.iter()
        .chain(rows.iter().rev().skip(1))
        .map(|s| s.to_string())
        .collect()
}
