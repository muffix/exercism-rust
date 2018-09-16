extern crate num_bigint;

use num_bigint::BigInt;
use std::cmp::{min, Ordering};
use std::ops::{Add, Mul, Sub};

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Eq, Debug)]
pub struct Decimal {
    mantissa: BigInt,
    exp: BigInt,
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Decimal) -> bool {
        if self.exp > other.exp {
            return self
                .pad_to_exp(other.exp.clone())
                .mantissa
                .eq(&other.mantissa);
        }

        self.mantissa
            .eq(&other.pad_to_exp(self.exp.clone()).mantissa)
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Decimal) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Decimal) -> Option<Ordering> {
        if self.exp > other.exp {
            return self
                .pad_to_exp(other.exp.clone())
                .mantissa
                .partial_cmp(&other.mantissa);
        }

        self.mantissa
            .partial_cmp(&other.pad_to_exp(self.exp.clone()).mantissa)
    }
}

impl Add for Decimal {
    type Output = Decimal;

    fn add(self, other: Decimal) -> Decimal {
        let mantissa;
        let exp = min(self.exp.clone(), other.exp.clone());

        if self.exp > other.exp {
            mantissa = self.pad_to_exp(other.exp).mantissa + other.mantissa;
        } else {
            mantissa = other.pad_to_exp(self.exp).mantissa + self.mantissa;
        }

        Decimal { mantissa, exp }.reduce()
    }
}

impl Sub for Decimal {
    type Output = Decimal;

    fn sub(self, other: Decimal) -> Decimal {
        let mantissa;
        let exp = min(self.exp.clone(), other.exp.clone());

        if self.exp > other.exp {
            mantissa = self.pad_to_exp(other.exp).mantissa - other.mantissa;
        } else {
            mantissa = self.mantissa - other.pad_to_exp(self.exp).mantissa;
        }

        Decimal { mantissa, exp }.reduce()
    }
}

impl Mul for Decimal {
    type Output = Decimal;

    fn mul(self, other: Decimal) -> Decimal {
        Decimal {
            mantissa: self.mantissa * other.mantissa,
            exp: self.exp + other.exp,
        }.reduce()
    }
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let dec_index = input.find('.');

        let exp = match dec_index {
            Some(idx) => -1 * BigInt::from(input.len() - idx - 1),
            None => BigInt::from(0),
        };

        let b = input
            .trim_left_matches('0')
            .chars()
            .filter_map(|c| {
                if c == '-' || c.is_numeric() {
                    Some(c as u8)
                } else {
                    None
                }
            }).collect::<Vec<u8>>();

        let mantissa = match b.len() {
            0 => BigInt::from(0),
            _ => BigInt::parse_bytes(&b, 10).unwrap(),
        };

        Some(Decimal { mantissa, exp }.reduce())
    }

    fn pad_to_exp(&self, exp: BigInt) -> Decimal {
        let mut mantissa = self.mantissa.clone();
        let mut current = self.exp.clone();
        while current > exp {
            mantissa *= 10;
            current -= 1;
        }

        Decimal { mantissa, exp }
    }

    fn reduce(&self) -> Decimal {
        let mut mantissa = self.mantissa.clone();
        let mut exp = self.exp.clone();
        let zero = BigInt::from(0);

        while mantissa.clone() % 10 == zero && mantissa != zero {
            mantissa /= 10;
            exp += 1;
        }

        Decimal { mantissa, exp }
    }
}
