fn is_prime(n: u32) -> bool {
    match n {
        0 => false,
        1 => false,
        2 => true,
        _ => {
            let mut p = 3;

            let limit = (n as f64).sqrt() as u32;

            while p <= limit {
                if n % p == 0 {
                    return false;
                }

                p = p + 1;
            }

            true
        }
    }
}

pub fn nth(n: u32) -> Option<u32> {
    const LIMIT: u32 = 200_000;
    let mut primes = [true; LIMIT as usize];

    primes[0] = false;

    for p in 1..LIMIT {
        if is_prime(p) {
            let mut i = 2;

            while p * i < LIMIT {
                primes[(p * i) as usize] = false;
                i = i + 1;
            }
        } else {
            primes[p as usize] = false;
        }
    }

    let mut i = 0;
    for (num, val) in primes.iter().enumerate() {
        if *val == true {
            i = i + 1;

            if i == n {
                return Some(num as u32);
            }
        }
    }

    None
}