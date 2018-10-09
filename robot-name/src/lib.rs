extern crate rand;

use rand::prelude::*;
use std::cell::RefCell;
use std::collections::HashSet;

thread_local!(static USED_NAMES: RefCell<HashSet<String>> = RefCell::new(HashSet::new()));

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Robot { name: new_name() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = new_name();
    }
}

fn new_name() -> String {
    fn generate_name() -> String {
        let mut rng = rand::thread_rng();

        let letters: String = (0..2).map(|_| rng.gen_range(b'A', b'Z') as char).collect();
        let numbers: String = (0..3)
            .map(|_| ('0' as u8 + rng.gen_range(0, 9) as u8) as char)
            .collect();

        format!("{}{}", letters, numbers)
    }

    USED_NAMES.with(|cell| {
        let mut used_names = cell.borrow_mut();
        let mut name = generate_name();

        while used_names.contains(&name) {
            name = generate_name();
        }

        used_names.insert(name.clone());
        name
    })
}
