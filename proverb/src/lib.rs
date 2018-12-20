#![allow(unused)]

pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        0 => return "".to_string(),
        1 => return format!("And all for the want of a {0}.", list.first().unwrap()),
        _ => {
            let mut proverbs: Vec<String> = Vec::with_capacity(list.len());
            let mut prev = list[0];

            for piece in &list[1..] {
                let proverb: String = format!("For want of a {0} the {1} was lost.", prev, piece);
                proverbs.push(proverb);
                prev = piece;
            }

            proverbs.push(format!(
                "And all for the want of a {0}.",
                list.first().unwrap()
            ));

            return proverbs.join("\n");
        }
    }
}
