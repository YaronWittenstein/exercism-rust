pub fn find(numbers: &[i32], number: i32) -> Option<i32> {
    if numbers.len() <= 0 {
        return None;
    }

    let mut start = 0;
    let mut end = (numbers.len() - 1) as i32;
    let mut res = 0;

    let mut found = false;

    while start <= end && found == false {
        let mid = (start + end) / 2;

        if mid >= numbers.len() as i32 {
            return None;
        }

        let mid_num: i32 = numbers[mid as usize];
        if mid_num == number {
            res = mid;
            found = true;
        } else if mid_num > number {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }

    match found {
        false => None,
        true => Some(res as i32),
    }
}