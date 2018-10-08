extern crate failure;

use failure::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// While using raw slice of str to handle flags is convenient,
/// in the real-world projects it is customary to use a struct,
/// that contains flags-related logic. So in this exercise
/// we ask you to implement a custom struct.
///
/// If you are curious about real-world implementation, refer to the `clap-rs` crate:
/// https://github.com/kbknapp/clap-rs/blob/master/src/args/arg_matches.rs
#[derive(Debug)]
pub struct Flags {
    matches: bool,
    case_sensitive: bool,
    partial: bool,
    line_numbers: bool,
    list: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        return Flags {
            line_numbers: flags.contains(&"-n"),
            case_sensitive: !flags.contains(&"-i"),
            partial: !flags.contains(&"-x"),
            list: flags.contains(&"-l"),
            matches: !flags.contains(&"-v"),
        };
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut matches = Vec::new();

    for f in files {
        matches.extend(grep_file(pattern, flags, f, files.len())?);
    }

    Ok(matches)
}

fn grep_file(
    pattern: &str,
    flags: &Flags,
    file: &str,
    total_files: usize,
) -> Result<Vec<String>, Error> {
    let mut matches = Vec::new();

    let f = File::open(file)?;
    for (l, line) in BufReader::new(f).lines().enumerate() {
        if let Ok(mut text) = line {
            let text_match = match (flags.partial, flags.case_sensitive) {
                (true, true) => text.contains(pattern),
                (true, false) => text
                    .to_lowercase()
                    .contains(pattern.to_lowercase().as_str()),
                (false, true) => text == pattern,
                (false, false) => text.to_lowercase() == pattern.to_lowercase(),
            };

            if flags.line_numbers {
                text = format!("{}:{}", l + 1, text);
            }
            if total_files > 1 {
                text = format!("{}:{}", file, text);
            }

            if !(text_match ^ flags.matches) {
                if flags.list {
                    matches.push(file.to_string());
                    break;
                } else {
                    matches.push(text);
                }
            }
        }
    }

    Ok(matches)
}
