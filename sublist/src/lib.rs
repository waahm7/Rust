use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let lengthOfFirstList = _first_list.len();
    let lengthOfSecondList = _second_list.len();
    
    if _first_list == _second_list {
        return Comparison::Equal
    }

    if lengthOfFirstList == 0 {
        return Comparison::Sublist
    }
    if lengthOfSecondList == 0 {
        return Comparison::Superlist
    }

    let differenceInLength = (lengthOfFirstList as i64 - lengthOfSecondList as i64).abs() as usize;
    if lengthOfFirstList < lengthOfSecondList {
        for i in 0..=differenceInLength {
            if _first_list == &_second_list[i..(i + lengthOfFirstList)] {
                return Comparison::Sublist
            }
        }
    } else {
        for i in 0..=differenceInLength {
            if _second_list == &_first_list[i..(i + lengthOfSecondList)] {
                return Comparison::Superlist
            }
        }
    }
    Comparison::Unequal
}
