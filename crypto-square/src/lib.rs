pub fn encrypt(input: &str) -> String {
    let mut input = input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<String>();
    let input_length = input.len();
    let (cols, rows) = dimensions(input_length);

    input.push_str(" ".repeat(rows * cols - input_length).as_str());

    let mut results = Vec::new();

    for r in 0..rows {
        results.push(input.chars().skip(r).step_by(rows).collect::<String>());
    }

    results.join(" ")
}

fn dimensions(length: usize) -> (usize, usize) {
    let rows = (length as f64).sqrt() as usize;
    let mut cols = rows;

    if rows * cols < length {
        cols += 1;
    }

    (rows, cols)
}
