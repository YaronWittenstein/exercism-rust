#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, n: usize) -> Result<u64, Error> {
    if n == 0 {
        return Ok(1);
    }

    string_digits
        .chars()
        .map(|c| c.to_digit(10).ok_or(Error::InvalidDigit(c)))
        .try_fold(Vec::new(), |mut acc, digit_or_err| -> _ {
            if digit_or_err.is_ok() {
                let digit = digit_or_err.unwrap();
                acc.insert(0, digit as u64);
                Ok(acc)
            } else {
                let err = digit_or_err.unwrap_err();
                Err(err)
            }
        })?
        .windows(n)
        .map(|w: &[u64]| w.iter().product::<u64>())
        .max()
        .ok_or(Error::SpanTooLong)
}
