#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    }

    if string_digits.len() < span {
        return Err(Error::SpanTooLong);
    }

    let non_numerics = string_digits
        .chars()
        .filter(|c| !c.is_numeric())
        .collect::<Vec<char>>();

    if non_numerics.len() > 0 {
        return Err(Error::InvalidDigit(non_numerics[0]));
    }

    Ok(string_digits
        .chars()
        .collect::<Vec<char>>()
        .windows(span)
        .map(|digits| {
            digits
                .iter()
                .map(|d| d.to_digit(10).unwrap())
                .fold(1, |acc, f| acc * f)
        })
        .collect::<Vec<u32>>()
        .iter()
        .fold(0, |max, x| u32::max(max, *x)) as u64)
}
