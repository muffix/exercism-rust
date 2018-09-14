extern crate regex;

use regex::Regex;

pub struct WordProblem {
    command: String,
}

impl WordProblem {
    pub fn new(command: &str) -> Self {
        WordProblem {
            command: command.to_string(),
        }
    }

    pub fn answer(&self) -> Option<i32> {
        let re = Regex::new(
            r"(?:What is (-?\d+))|(?:(multiplied|divided|plus|minus) (?:by )?(-?\d+))",
        ).unwrap();

        if !re.is_match(&self.command) {
            return None;
        }

        let mut matches = re.captures_iter(&self.command);
        let mut result: i32 = matches.next().unwrap()[1].parse().unwrap();
        let mut in_init_state = true;

        for m in matches {
            let op = &m[2];
            let operand: i32 = m[3].parse().unwrap();

            match op {
                "plus" => result += operand,
                "minus" => result -= operand,
                "multiplied" => result *= operand,
                "divided" => result /= operand,
                _ => return None,
            }
            in_init_state = false;
        }

        if in_init_state { None } else { Some(result) }
    }
}
