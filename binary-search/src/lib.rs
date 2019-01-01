use std::cmp::Ord;
use std::cmp::Ordering;

pub fn find<T, R>(items: R, val: T) -> Option<i32>
where
    R: AsRef<[T]>,
    T: Ord,
{
    let items: &[T] = items.as_ref();

    if items.len() == 0 {
        return None;
    }

    let mut start: isize = 0;
    let mut end: isize = (items.len() - 1) as isize;

    while start <= end {
        let mid = (start + end) / 2;

        let mid_val = items.get(mid as usize).unwrap();

        match mid_val.cmp(&val) {
            Ordering::Equal => return Some(mid as i32),
            Ordering::Greater => end = mid - 1,
            Ordering::Less => start = mid + 1,
        }
    }

    None
}
