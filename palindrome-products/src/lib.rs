#[macro_use]
extern crate itertools;

use std::collections::HashSet;

pub type Palindrome = u64;
pub fn get_palindrome_products(min: u64, max: u64) -> Vec<Palindrome> {
    iproduct!((min..max + 1), (min..max + 1))
        .filter_map(|(a, b)| to_palindrome(a * b))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

pub fn min(palindromes: &[Palindrome]) -> Option<Palindrome> {
    palindromes.into_iter().fold(None, |min, &p| match min {
        None => Some(p),
        Some(current_min) => Some(if p < current_min { p } else { current_min }),
    })
}

pub fn max(palindromes: &[Palindrome]) -> Option<Palindrome> {
    palindromes.into_iter().fold(None, |min, &p| match min {
        None => Some(p),
        Some(current_max) => Some(if p > current_max { p } else { current_max }),
    })
}

fn to_palindrome(num: u64) -> Option<Palindrome> {
    let num_str = num.to_string();

    let half_len = num_str.len() / 2;
    if num_str
        .chars()
        .take(half_len)
        .eq(num_str.chars().rev().take(half_len))
    {
        return Some(num as Palindrome);
    }

    None
}
