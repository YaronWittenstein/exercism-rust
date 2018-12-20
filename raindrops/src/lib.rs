pub fn raindrops(n: usize) -> String {
    let mut strings: Vec<String> = Vec::new();

    if n % 3 == 0 {
        strings.push(String::from("Pling"));
    }

    if n % 5 == 0 {
        strings.push(String::from("Plang"));
    }

    if n % 7 == 0 {
        strings.push(String::from("Plong"));
    }

    if strings.len() < 1 {
        strings.push(n.to_string());
    }

    strings.join("")
}