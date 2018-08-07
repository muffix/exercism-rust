#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Unequal,
    Sublist,
    Superlist,
}

pub fn sublist<T: PartialEq>(short_list: &[T], long_list: &[T]) -> Comparison {
    if short_list.len() == 0 {
        if long_list.len() == 0 {
            return Comparison::Equal;
        }
        return Comparison::Sublist;
    }

    if short_list.len() > long_list.len() {
        let result = sublist(long_list, short_list);
        if result == Comparison::Sublist {
            return Comparison::Superlist;
        } else {
            return result;
        }
    }

    let is_sublist = long_list
        .windows(short_list.len())
        .any(|list| list == short_list);

    if is_sublist {
        if short_list.len() == long_list.len() {
            return Comparison::Equal;
        } else {
            return Comparison::Sublist;
        }
    }

    Comparison::Unequal
}
