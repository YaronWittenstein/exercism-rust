pub fn sing(start: i32, end: i32) -> String {
    let mut items: Vec<String> = (end..start + 1).map(|n| verse(n)).collect();

    (&mut items).reverse();

    items.join("\n")
}

pub fn verse(n: i32) -> String {
    format!(
        "{0} of beer on the wall, {1} of beer.\n{2}, {3} of beer on the wall.\n",
        pluralize_bottles(n, true),
        pluralize_bottles(n, false),
        next_step(n),
        pluralize_bottles(n - 1, false),
    )
}

fn pluralize_bottles(n: i32, capitalize: bool) -> String {
    match n {
        -1 => "99 bottles".to_string(),
        0 => format!("{0} more bottles", if capitalize { "No" } else { "no" }),
        1 => "1 bottle".to_string(),
        _ => format!("{0} bottles", n),
    }
}

fn next_step(n: i32) -> String {
    match n {
        0 => "Go to the store and buy some more".to_string(),
        1 => "Take it down and pass it around".to_string(),
        _ => "Take one down and pass it around".to_string(),
    }
}
