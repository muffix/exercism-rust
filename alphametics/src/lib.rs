extern crate itertools;

use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let components: Vec<&str> = input.split(" == ").collect();

    let (left, target) = (components[0], components[1]);
    let summands: Vec<&str> = left.split(" + ").collect();

    let mut letters = HashSet::new();

    for letter in input.chars() {
        if letter.is_ascii_uppercase() {
            letters.insert(letter);
        }
    }

    let letters = letters.into_iter().collect::<Vec<char>>();

    for combi in (0..10).combinations(letters.len()) {
        for perm in permutations(combi) {
            println!("Letters {:?}", letters);
            println!("Perm    {:?}", perm);
            let candidate: HashMap<char, u8> = letters.iter().cloned().zip(perm).collect();
            if validate(&candidate, &summands, target) {
                return Some(candidate);
            }
        }
    }

    None
}

fn validate(candidate: &HashMap<char, u8>, summands: &[&str], target: &str) -> bool {
    if summands
        .iter()
        .any(|summand| *candidate.get(&summand.chars().next().unwrap()).unwrap() == 0)
        || *candidate.get(&target.chars().next().unwrap()).unwrap() == 0
    {
        return false;
    }

    let sum: u32 = summands
        .iter()
        .map(|summand| convert_chars(summand, candidate))
        .sum();

    let target_value = convert_chars(target, candidate);

    println!("Sum: {}, target: {}", sum, target_value);
    sum == target_value
}

fn convert_chars(chars: &str, mapping: &HashMap<char, u8>) -> u32 {
    chars
        .chars()
        .map(|d| mapping[&d].to_string())
        .collect::<String>()
        .parse::<u32>()
        .unwrap()
}

pub fn permutations(items: Vec<u8>) -> Permutations {
    Permutations {
        idxs: items.clone(),
        swaps: vec![0; items.len()],
        i: 0,
    }
}

pub struct Permutations {
    idxs: Vec<u8>,
    swaps: Vec<usize>,
    i: usize,
}

impl Iterator for Permutations {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i > 0 {
            loop {
                if self.i >= self.swaps.len() {
                    return None;
                }
                if self.swaps[self.i] < self.i {
                    break;
                }
                self.swaps[self.i] = 0;
                self.i += 1;
            }
            self.idxs.swap(self.i, (self.i & 1) * self.swaps[self.i]);
            self.swaps[self.i] += 1;
        }
        self.i = 1;
        Some(self.idxs.clone())
    }
}
