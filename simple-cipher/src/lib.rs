extern crate rand;

use rand::prelude::*;
use std::cmp::max;

enum Operation {
    Encode,
    Decode,
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    vigenere(key, s, Operation::Encode)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    vigenere(key, s, Operation::Decode)
}

pub fn encode_random(s: &str) -> (String, String) {
    let key = (0..max(100, s.len()))
        .map(|_| (rand::thread_rng().gen_range::<u8>(0, 26) + 'a' as u8) as char)
        .collect::<String>();

    let encoded = encode(&key[..], s).unwrap();
    (key, encoded)
}

fn shift_char(c: char, offset: u8, op: &Operation) -> char {
    let c = match op {
        Operation::Encode => c as u8 + offset,
        Operation::Decode => c as u8 - offset,
    };

    if c > 'z' as u8 {
        return (c as u8 - 26) as char;
    } else if c < 'a' as u8 {
        return (c as u8 + 26) as char;
    }

    c as char
}

fn vigenere(key: &str, s: &str, op: Operation) -> Option<String> {
    if key.len() == 0 || !key.chars().all(|c| c.is_ascii_lowercase()) {
        return None;
    }

    let mut key = String::from(key);

    if key.len() < s.len() {
        key = std::iter::repeat(key).take(s.len()).collect();
    }

    Some(
        s.chars()
            .zip(key.chars())
            .map(|(c, k)| shift_char(c, k as u8 - 'a' as u8, &op))
            .collect(),
    )
}
