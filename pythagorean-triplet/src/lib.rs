pub fn find() -> Option<u32> {
    for a in 1..1000 {
        for b in (a + 1)..(1000 - a) {
            let c = 1000 - (a + b);

            if c > b && is_triplet(a, b, c) {
                return Some(a * b * c);
            }
        }
    }

    None
}

fn is_triplet(a: u32, b: u32, c: u32) -> bool {
    a * a + b * b == c * c
}
