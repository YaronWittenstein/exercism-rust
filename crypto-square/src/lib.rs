struct Rectangle {
    r: usize,
    c: usize,
    pad: usize,
}

pub fn encrypt(input: &str) -> String {
    let chars = input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_digit(10))
        .collect::<Vec<_>>();

    if chars.len() == 0 {
        return String::from("");
    }

    let dims = calc_rectangle(chars.len());
    let pads = std::iter::repeat(' ').take(dims.pad);

    let chars_vec = chars.into_iter().chain(pads).collect::<Vec<_>>();
    let mut strings = Vec::<String>::with_capacity(dims.c);

    (0..dims.c).for_each(|c| {
        let mut entry: Vec<char> = Vec::with_capacity(dims.r);

        (0..dims.r).for_each(|r| {
            let ch = chars_vec[r * dims.c + c];
            entry.push(ch);
        });

        strings.push(entry.iter().collect());
    });

    strings.join(" ")
}

fn calc_rectangle(n: usize) -> Rectangle {
    let (c, r) = (n..).find_map(|c| find_pair(c)).unwrap();

    Rectangle {
        c,
        r,
        pad: c * r - n,
    }
}

fn find_pair(n: usize) -> Option<(usize, usize)> {
    (1..n).find_map(|c| {
        if c * c == n {
            Some((c, c))
        } else if c * (c - 1) == n {
            Some((c, c - 1))
        } else {
            None
        }
    })
}
