#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values
        .iter()
        .map(|num| number_to_bytes(*num))
        .collect::<Vec<Vec<u8>>>()
        .iter()
        .flat_map(|ref a| a.iter())
        .map(|b| *b)
        .collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut res = Vec::new();
    let mut bytes_for_num = Vec::new();

    for b in bytes {
        bytes_for_num.push(*b);

        if *b < 0x80 {
            let len = bytes_for_num.len();
            let number = bytes_for_num
                .iter()
                .enumerate()
                .map(|(i, b)| ((*b & 0x7F) as u64) << (len - i - 1) * 7)
                .fold(0, |acc, x| acc | x);

            if number > std::u32::MAX as u64 {
                return Err(Error::Overflow);
            }

            res.push(number as u32);

            bytes_for_num = Vec::new();
        }
    }

    if bytes_for_num.len() > 0 {
        return Err(Error::IncompleteNumber);
    }

    Ok(res)
}

fn number_to_bytes(mut num: u32) -> Vec<u8> {
    let mut res = Vec::new();

    if num == 0 {
        res.push(0);
        return res;
    }

    while num > 0 {
        let mut byte = (num % 0x80) as u8;
        if res.len() > 0 {
            byte |= 0x80;
        }
        res.insert(0, byte);
        num >>= 7;
    }

    res
}
