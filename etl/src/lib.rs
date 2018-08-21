use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .map(|(score, chars)| {
            chars
                .into_iter()
                .map(|c| (c.to_ascii_lowercase(), *score))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .concat()
        .into_iter()
        .collect()
}
