pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }

    let diff = s1
        .chars()
        .zip(s2.chars())
        .fold(0, |acc, (c, d)| if c == d { acc } else { acc + 1 });

    Some(diff)
}
