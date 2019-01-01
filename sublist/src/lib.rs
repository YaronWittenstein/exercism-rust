#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let sublist1 = is_sublist(first_list, second_list);
    let sublist2 = is_sublist(second_list, first_list);

    match (sublist1, sublist2) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}

fn is_sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    let wsize = first_list.len();

    match wsize {
        0 => true,
        _ => second_list.windows(wsize).any(|w| w == first_list),
    }
}
