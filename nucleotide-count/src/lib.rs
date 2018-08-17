use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::iter::FromIterator;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match try!(nucleotide_counts(dna)).get(&nucleotide) {
        None => Err(nucleotide),
        Some(n) => Ok(*n),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::from_iter(vec![('G', 0), ('T', 0), ('A', 0), ('C', 0)]);

    for c in dna.chars() {
        match counts.entry(c) {
            Entry::Vacant(_) => return Err(c),
            Entry::Occupied(n) => *n.into_mut() += 1,
        }
    }

    Ok(counts)
}
