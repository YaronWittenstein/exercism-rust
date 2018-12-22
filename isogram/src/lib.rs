use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let chars1 = candidate
        .chars()
        .flat_map(|c| c.to_lowercase().collect::<Vec<char>>())
        .filter(|c| *c != '-' && *c != ' ')
        .collect::<Vec<char>>();

    let chars2 = chars1.clone().into_iter().collect::<HashSet<char>>();

    chars1.len() == chars2.len()
}
