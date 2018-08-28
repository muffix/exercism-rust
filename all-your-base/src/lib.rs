#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits. However, your function must be able to
///     process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, target_base: u32) -> Result<Vec<u32>, Error> {
    if target_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }

    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }

    match to_decimal(from_base, number) {
        Ok(n) => Ok(to_base(target_base, n)),
        Err(d) => Err(Error::InvalidDigit(d)),
    }
}

fn to_decimal(base: u32, digits: &[u32]) -> Result<u32, u32> {
    let mut result = 0;
    for d in digits.iter() {
        if d >= &base {
            return Err(*d);
        }
        result = result * base + d;
    }
    Ok(result)
}

fn to_base(base: u32, mut number: u32) -> Vec<u32> {
    let mut result = Vec::new();
    while number > 0 {
        result.insert(0, number % base);
        number = number / base;
    }
    result
}
