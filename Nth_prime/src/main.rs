pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = Vec::new();
    let mut num = 2;

    while primes.len() <= n as usize {
        if is_prime(num) {
            primes.push(num);
        }
        num += 1;
    }

    primes[n as usize]
}

fn is_prime(x: u32) -> bool {
    if x < 2 {
        return false;
    }

    for i in 2..=((x as f64).sqrt() as u32) {
        if x % i == 0 {
            return false;
        }
    }

    true
}

fn main() {}