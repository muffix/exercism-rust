use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower_word = word.to_lowercase();
    let letters = sorted_letters(word);

    possible_anagrams
        .into_iter()
        .filter_map(|&candidate| {
            let candidate_letters = sorted_letters(candidate);
            if letters == candidate_letters && lower_word != candidate.to_lowercase() {
                Some(candidate)
            } else {
                None
            }
        }).collect()
}

fn sorted_letters(word: &str) -> Vec<char> {
    let mut letters: Vec<char> = word.to_lowercase().chars().collect();
    letters.sort();
    letters
}
