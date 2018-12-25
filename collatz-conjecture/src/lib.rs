pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut steps = 0;
    let mut m = n;

    while m != 1 {
        steps += 1;

        if m % 2 == 0 {
            m = m / 2;
        } else {
            m = 3 * m + 1;
        }
    }

    Some(steps)
}
