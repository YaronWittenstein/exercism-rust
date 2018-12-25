pub fn is_valid(code: &str) -> bool {
    let valid = code.chars().all(|c| c.is_digit(10) || c.is_whitespace());

    if !valid {
        return false;
    }

    let stripped_code: Vec<_> = code.chars().rev().filter(|c| c.is_digit(10)).collect();

    if stripped_code.len() < 2 {
        return false;
    }

    let sum: u32 = stripped_code.iter().enumerate().fold(0, |acc, (i, c)| {
        let mut d = c.to_digit(10).unwrap();

        if i % 2 == 1 {
            d = d * 2;

            if d > 9 {
                d = d - 9;
            }
        }

        acc + d
    });

    sum % 10 == 0
}
