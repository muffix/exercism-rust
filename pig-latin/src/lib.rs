#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| to_pig_latin(word))
        .collect::<Vec<_>>()
        .join(" ")
}

fn to_pig_latin(word: &str) -> String {
    lazy_static! {
        static ref starts_with_vowel_re: Regex = Regex::new(r"^(xr|yt|[aeiou]+)(.*)$").unwrap();
        static ref y_after_consonant_re: Regex = Regex::new(r"^([^aeiouy]+)((y).*)").unwrap();
        static ref qu_after_consonant_re: Regex = Regex::new(r"^([^aeiou]*qu)(.*)").unwrap();
        static ref starts_with_consonant_re: Regex = Regex::new(r"^([^aeiou]*)(.*)$").unwrap();
    }

    if starts_with_vowel_re.is_match(word) {
        return format!("{}ay", word);
    }

    let mut matches = y_after_consonant_re.captures_iter(word).next();

    if matches.is_none() {
        matches = qu_after_consonant_re.captures_iter(word).next();
        if matches.is_none() {
            matches = starts_with_consonant_re.captures_iter(word).next();
        }
    }

    let matches = matches.unwrap();

    format!(
        "{}{}ay",
        matches.get(2).unwrap().as_str(),
        matches.get(1).unwrap().as_str()
    )
}
