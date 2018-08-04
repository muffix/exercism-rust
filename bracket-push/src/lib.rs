use std::collections::VecDeque;

pub struct Brackets<'a> {
    string: &'a str,
}

static BRACKETS: &[(char, char)] = &[('(', ')'), ('[', ']'), ('{', '}')];

impl<'a> From<&'a str> for Brackets<'a> {
    fn from(input: &'a str) -> Self {
        Brackets { string: input }
    }
}

impl<'a> Brackets<'a> {
    pub fn are_balanced(&self) -> bool {
        let mut stack = VecDeque::new();

        for c in self.string.chars() {
            if let Some(b) = closing(c) {
                stack.push_back(b)
            } else if stack.back() == Some(&c) {
                stack.pop_back();
            } else if is_closing(c) {
                return false;
            }
        }

        stack.is_empty()
    }
}

fn closing(bracket: char) -> Option<char> {
    BRACKETS
        .iter()
        .find(|(open, _)| *open == bracket)
        .map(|(_, close)| *close)
}

fn is_closing(bracket: char) -> bool {
    BRACKETS.iter().any(|&(_, close)| close == bracket)
}
