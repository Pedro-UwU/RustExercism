#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    // We have 3 cases
    let diff: i64 = first_list.len() as i64 - second_list.len() as i64;
    if diff == 0 {
        return if first_list.eq(second_list) { Comparison::Equal } else { Comparison::Unequal }
    }

    //if fist is larger than second
    if diff > 0 {
        return if is_sublist(second_list, first_list) { Comparison::Superlist } else { Comparison::Unequal }
    }

    return if is_sublist(first_list, second_list) { Comparison::Sublist } else { Comparison::Unequal }
}

// a.len() < b.len() always
pub fn is_sublist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    let diff = b.len() - a.len();
    for i in 0..=diff {
        let mut equal = true;
        for j in 0..a.len() {
            if a[j] != b[i+j] {
                equal = false;
                break; 
            }
        }
        if equal {
            return true
        }
    };
    false
}
