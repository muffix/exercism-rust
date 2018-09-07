#[macro_use]
extern crate enum_primitive;
extern crate num;

use num::FromPrimitive;

enum_from_primitive! {
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}
}

pub struct Allergies {
    code: u32,
}

impl Allergies {
    pub fn new(code: u32) -> Allergies {
        Allergies { code: code }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.code & 1 << (*allergen as u32) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        (0..8)
            .map(|n| Allergen::from_u32(n).unwrap())
            .filter(|a| self.is_allergic_to(&a))
            .collect()
    }
}
