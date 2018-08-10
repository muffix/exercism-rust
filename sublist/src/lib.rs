#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Unequal,
    Sublist,
    Superlist,
}

pub fn sublist<T: PartialEq>(list_a: &[T], list_b: &[T]) -> Comparison {
    match (list_a.len(), list_b.len()) {
        (a, b) if a == b && is_sublist(list_a, list_b) => Comparison::Equal,
        (a, b) if a < b && is_sublist(list_a, list_b) => Comparison::Sublist,
        (a, b) if a > b && is_sublist(list_b, list_a) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}

fn is_sublist<T: PartialEq>(short_list: &[T], long_list: &[T]) -> bool {
    short_list.is_empty() || long_list.windows(short_list.len()).any(|list| list == short_list)
}
