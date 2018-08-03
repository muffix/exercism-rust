use std::collections::LinkedList;

pub struct Brackets {
    string: String,
}

static BRACKETS: &[(char, char)] = &[('(', ')'), ('[', ']'), ('{', '}')];

impl<'a> From<&'a str> for Brackets {
    fn from(input: &str) -> Self {
        Brackets {
            string: input.to_owned(),
        }
    }
}

impl Brackets {
    pub fn are_balanced(&self) -> bool {
        let mut stack: LinkedList<char> = LinkedList::new();

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
    for &(open, close) in BRACKETS {
        if open == bracket {
            return Some(close);
        }
    }
    None
}

fn is_closing(bracket: char) -> bool {
    BRACKETS.iter().any(|&(_, close)| close == bracket)
}
