pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut digit:Vec<String> = Vec::new();
    if len > digits.len() || len == 0{
        return digit;
    }
    if len == digits.len(){
        digit.push(digits.to_string());
        return digit;
    }

    for i in 0..= digits.len() - len{
        let sub = &digits[i..i+len];
        digit.push(sub.to_string());
    }
    digit
}

fn main() {}