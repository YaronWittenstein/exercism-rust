pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits = Vec::<u32>::new();
    let mut n = num;

    while n > 0 {
        digits.push(n % 10);
        n = n / 10;
    }

    let ndigits = digits.len() as u32;

    let sum: u32 = digits.iter().map(|d| d.pow(ndigits) as u32).sum();

    sum == num
}
