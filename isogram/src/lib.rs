use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut seen: HashSet<char> = HashSet::new();

    for c in candidate.to_lowercase().chars() {
        if c == ' ' || c == '-' {
            continue;
        }

        if seen.contains(&c) {
            return false;
        }

        seen.insert(c);
    }

    true
}
