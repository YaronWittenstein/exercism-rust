pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        0 => return "".to_string(),
        _ => {
            let last_piece = format!("And all for the want of a {0}.", list[0]);

            list.windows(2)
                .map(|w| format!("For want of a {0} the {1} was lost.", w[0], w[1]))
                .chain(vec![last_piece])
                .collect::<Vec<_>>()
                .join("\n")
        }
    }
}
