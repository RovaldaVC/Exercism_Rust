/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // remove spaces and validate characters
    let mut cleaned = String::with_capacity(code.len());
    for c in code.chars() {
        if c.is_whitespace() { continue; }
        if !c.is_ascii_digit() { return false; }
        cleaned.push(c);
    }

    // must contain at least two digits
    if cleaned.len() <= 1 { return false; }

    // compute Luhn checksum
    let sum: u32 = cleaned
        .chars()
        .rev()
        .enumerate()
        .map(|(i, ch)| {
            let mut n = ch.to_digit(10).unwrap();
            if i % 2 == 1 {
                n *= 2;
                if n > 9 { n -= 9; }
            }
            n
        })
        .sum();

    sum % 10 == 0
}

fn main(){}