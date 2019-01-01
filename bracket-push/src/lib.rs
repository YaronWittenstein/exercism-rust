pub fn brackets_are_balanced(string: &str) -> bool {
    string
        .chars()
        .filter(is_bracket)
        .fold(Vec::new(), |mut brackets, b| match brackets.len() {
            0 => {
                brackets.push(b);
                brackets
            }
            _ => {
                let top = brackets.pop().unwrap();

                if !is_pair(top, b) {
                    brackets.push(top);
                    brackets.push(b);
                }

                brackets
            }
        })
        .is_empty()
}

fn is_pair(b1: char, b2: char) -> bool {
    match (b1, b2) {
        ('(', ')') | ('[', ']') | ('{', '}') => true,
        _ => false,
    }
}

fn is_bracket(b: &char) -> bool {
    match b {
        '(' | ')' | '[' | ']' | '{' | '}' => true,
        _ => false,
    }
}
