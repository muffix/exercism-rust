use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut count = HashMap::new();

    let filtered: String = words
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect();

    for word in filtered.split_whitespace() {
        count
            .entry(word.to_string())
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    count
}
