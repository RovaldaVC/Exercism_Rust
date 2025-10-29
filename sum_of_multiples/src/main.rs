use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut no_dup = HashSet::new();
        
    for &i in factors{
        if i as u32 == 0{
            continue;
        }
        let mut num:u32 = i;
        while num < limit {
            no_dup.insert(num);
        num +=i;
        }
    }

    no_dup.iter().sum()
}

fn main() {}