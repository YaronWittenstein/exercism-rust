use std::collections::HashMap;

pub fn encode(plain: &str) -> String {
    transform(plain)
        .chars()
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<_>())
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn decode(cipher: &str) -> String {
    transform(cipher)
}

pub fn transform(text: &str) -> String {
    let chr_map = build_table();

    text.to_lowercase()
        .chars()
        .filter(|c| (c.is_ascii() && c.is_alphabetic()) || c.is_digit(10))
        .map(|c| {
            if let Some(&matched) = chr_map.get(&c) {
                matched
            } else {
                c
            }
        })
        .collect::<String>()
}

fn build_table() -> HashMap<char, char> {
    let mut chr_map = HashMap::new();

    (0..25).for_each(|i| {
        let chr = char::from(b'a' + i);
        let rev = char::from(b'z' - i);

        chr_map.insert(chr, rev);
        chr_map.insert(rev, chr);
    });

    chr_map
}
