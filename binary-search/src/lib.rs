#![allow(unused)]

pub fn find(numbers: &[i32], number: i32) -> Option<i32> {
    if numbers.len() == 0 {
        return None;
    }

    let mut start: isize = 0;
    let mut end: isize = (numbers.len() - 1) as isize;

    while start <= end {
        let mid = (start + end) / 2;

        let v: i32 = numbers[mid as usize];

        if v == number {
            return Some(mid as i32);
        }

        if number < v {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }

    None
}
