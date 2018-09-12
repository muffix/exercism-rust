use std::fmt::Debug;

#[derive(Debug)]
pub struct CustomSet<T: PartialEq + Clone> {
    elements: Vec<T>,
}

impl<T: Debug + PartialEq + Clone> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut set = CustomSet { elements: vec![] };
        for element in input {
            set.add(element.clone());
        }
        set
    }

    pub fn contains(&self, element: &T) -> bool {
        self.elements.contains(element)
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.elements.push(element);
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.elements.iter().all(|e| other.contains(e))
    }

    pub fn is_empty(&self) -> bool {
        self.elements.len() == 0
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.intersection(other).is_empty()
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let mut set = CustomSet { elements: vec![] };

        for e in self.elements.iter().cloned() {
            if other.contains(&e) {
                set.add(e);
            }
        }

        set
    }

    pub fn difference(&self, other: &Self) -> Self {
        let mut set = CustomSet { elements: vec![] };

        for e in self.elements.iter().cloned() {
            if !other.contains(&e) {
                set.add(e);
            }
        }

        set
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut union = Self::new(&vec![]);
        for item in self.elements.iter().chain(other.elements.iter()).cloned() {
            union.add(item);
        }
        union
    }
}

impl<T: Debug + Clone> PartialEq for CustomSet<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &CustomSet<T>) -> bool {
        self.is_subset(other) && other.is_subset(self)
    }
}
