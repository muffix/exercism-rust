use std::collections::HashMap;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower_word = word.to_lowercase();
    let letters = letters_count(word);

    possible_anagrams
        .into_iter()
        .filter_map(|&candidate| {
            match letters == letters_count(candidate) && lower_word != candidate.to_lowercase() {
                true => Some(candidate),
                false => None,
            }
        }).collect()
}

fn letters_count(word: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for c in word.to_lowercase().chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    map
}
