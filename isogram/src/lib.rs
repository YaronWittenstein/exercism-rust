use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let chars1 = candidate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<_>>();

    let chars2 = chars1.clone().into_iter().collect::<HashSet<_>>();

    chars1.len() == chars2.len()
}
