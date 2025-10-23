pub fn square(s: u32) -> u64 {
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    let mut sum = 0;
    for i in 1..=64 {
        sum += square(i);
    }
    sum
}

fn main() {
    println!("Hello, world!");
}
