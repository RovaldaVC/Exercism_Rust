#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    // base checks first
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }
    if number.is_empty() {
    return Ok(vec![0]);
}

    // digit validation
    let mut value = 0;
    for &digit in number {
        if digit >= from_base {
            return Err(Error::InvalidDigit(digit));
        }
        //convert from base to demical.
        value = value * from_base + digit;
    }
    if value == 0{
        return Ok(vec![0]);
    }
    let mut list = Vec::new();
    while value > 0{
        let remainder = value % to_base;
        value = value / to_base;
        list.push(remainder);
    }
    list.reverse();
    Ok(list)
}

fn main(){}