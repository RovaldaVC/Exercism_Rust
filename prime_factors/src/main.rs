pub fn factors(mut n: u64) -> Vec<u64> {
    let mut res:Vec<u64> = Vec::new();
    let mut d = 2u64;

    while n > 1{
        while n % d == 0{
            res.push(d);
            n /= d;
        }
        d += 1;
        // small optimization: stop if d*d > n, then n is prime
        if d * d > n {
            if n > 1{
                res.push(n);
            }
            break;
        }
    }
    res    
}


fn main() {}