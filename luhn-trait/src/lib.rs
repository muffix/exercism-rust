pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T> Luhn for T
where
    T: ToString,
{
    fn valid_luhn(&self) -> bool {
        let filtered = self
            .to_string()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();

        if filtered.chars().any(|c| !c.is_digit(10)) || filtered.len() < 2 {
            return false;
        }

        filtered
            .chars()
            .rev()
            .filter_map(|c| c.to_digit(10))
            .enumerate()
            .map(|(i, d)| if i % 2 == 1 { luhn_double(d) } else { d })
            .sum::<u32>() % 10 == 0
    }
}

fn luhn_double(d: u32) -> u32 {
    if d * 2 > 9 {
        d * 2 - 9
    } else {
        d * 2
    }
}
