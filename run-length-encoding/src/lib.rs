pub fn encode(source: &str) -> String {
    let mut substrings = Vec::<String>::new();
    let mut state: Option<(usize, char)> = None;

    for c in source.chars() {
        state = update_state(c, &mut substrings, state);
    }

    // we can assume that `source` will not contain numbers
    // this will trigger encoding of the last sub-string
    update_state('1', &mut substrings, state);

    substrings.join("")
}

pub fn decode(source: &str) -> String {
    let mut digits = Vec::<char>::new();
    let mut substrings = Vec::<String>::new();

    for c in source.chars() {
        if c.is_digit(10) {
            digits.push(c);
        } else {
            match digits.len() {
                0 => substrings.push(format!("{}", c)),
                _ => {
                    let number = digits.iter().collect::<String>().parse::<usize>().unwrap();
                    let digits_str = std::iter::repeat(c).take(number).collect::<String>();
                    substrings.push(digits_str);
                    digits.clear();
                }
            }
        }
    }

    substrings.join("")
}

fn update_state(
    c: char,
    substrings: &mut Vec<String>,
    state: Option<(usize, char)>,
) -> Option<(usize, char)> {
    match state {
        None => Some((1, c)),
        Some((len, d)) if c == d => Some((len + 1, c)),
        Some((len, d)) => {
            let substr = match len {
                1 => format!("{}", d),
                _ => format!("{}{}", len, d),
            };

            substrings.push(substr);

            Some((1, c))
        }
    }
}
