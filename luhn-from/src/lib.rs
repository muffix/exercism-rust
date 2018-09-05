pub struct Luhn {
    digits: Option<Vec<u32>>,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        if self.digits.is_none() {
            return false;
        }

        self.digits
            .clone()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(i, d)| {
                if i % 2 == 1 {
                    Luhn::luhn_double(*d)
                } else {
                    *d
                }
            })
            .sum::<u32>() % 10 == 0
    }

    fn luhn_double(d: u32) -> u32 {
        if d * 2 > 9 {
            d * 2 - 9
        } else {
            d * 2
        }
    }
}

impl From<String> for Luhn {
    fn from(code: String) -> Luhn {
        let filtered = code
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<Vec<char>>();

        if filtered.iter().any(|c| !c.is_digit(10)) || filtered.len() < 2 {
            return Luhn { digits: None };
        }

        let digits = filtered
            .iter()
            .rev()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>();

        Luhn {
            digits: Some(digits),
        }
    }
}

impl From<&'static str> for Luhn {
    fn from(characters: &str) -> Luhn {
        Luhn::from(characters.to_string())
    }
}

impl From<u8> for Luhn {
    fn from(code: u8) -> Luhn {
        Luhn::from(code.to_string())
    }
}

impl From<u16> for Luhn {
    fn from(code: u16) -> Luhn {
        Luhn::from(code.to_string())
    }
}

impl From<u32> for Luhn {
    fn from(code: u32) -> Luhn {
        Luhn::from(code.to_string())
    }
}

impl From<u64> for Luhn {
    fn from(code: u64) -> Luhn {
        Luhn::from(code.to_string())
    }
}

impl From<usize> for Luhn {
    fn from(code: usize) -> Luhn {
        Luhn::from(code.to_string())
    }
}
