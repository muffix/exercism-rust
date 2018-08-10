#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Unequal,
    Sublist,
    Superlist,
}

pub fn sublist<T: PartialEq>(list_a: &[T], list_b: &[T]) -> Comparison {
    let mut swapped = false;
    let (mut short_list, mut long_list) = (list_a, list_b);

    if list_a.len() > list_b.len() {
        long_list = list_a;
        short_list = list_b;
        swapped = true;
    }

    is_sublist(short_list, long_list, swapped)
}

fn is_sublist<T: PartialEq>(short_list: &[T], long_list: &[T], swapped: bool) -> Comparison {
    if short_list.len() == 0 {
        if long_list.len() == 0 {
            return Comparison::Equal;
        }

        return if swapped { Comparison::Superlist } else { Comparison::Sublist };
    }

    let is_sublist = long_list
        .windows(short_list.len())
        .any(|list| list == short_list);

    if is_sublist {
        if short_list.len() == long_list.len() {
            return Comparison::Equal;
        } else {
            return if swapped { Comparison::Superlist } else { Comparison::Sublist };
        }
    }

    Comparison::Unequal
}
