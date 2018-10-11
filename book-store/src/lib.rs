use std::collections::HashSet;

pub fn lowest_price(books: &[u32]) -> u32 {
    groups(books.to_vec())
        .iter()
        .map(discounted_price)
        .min_by(|x, y| x.cmp(y))
        .unwrap_or(0)
}

fn groups<T>(items: Vec<T>) -> Vec<Vec<HashSet<T>>>
where
    T: Eq + Clone + std::hash::Hash,
{
    let mut results: Vec<Vec<HashSet<T>>> = vec![vec![]];

    for item in &items {
        let mut next_results = Vec::new();

        for result in &results {
            for (s, set) in result.iter().enumerate() {
                if !set.contains(item) {
                    let mut iteration = result.clone();
                    iteration[s].insert(item.clone());
                    next_results.push(iteration)
                }
            }

            let mut iteration = result.clone();
            let mut new_set = HashSet::new();

            new_set.insert(item.clone());
            iteration.push(new_set);
            next_results.push(iteration);
        }

        results = next_results;
    }

    results
}

fn discounted_price(sets: &Vec<HashSet<u32>>) -> u32 {
    let mut total = 0.0;
    for set in sets {
        total += match set.len() {
            0 => 0.0,
            1 => 800.0,
            2 => 800.0 * 2.0 * 0.95,
            3 => 800.0 * 3.0 * 0.90,
            4 => 800.0 * 4.0 * 0.80,
            5 => 800.0 * 5.0 * 0.75,
            _ => panic!(),
        }
    }
    total as u32
}
