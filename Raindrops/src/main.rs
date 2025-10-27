pub fn raindrops(n: u32) -> String {
    let mut text = String::new();
    if n % 3 == 0{
        text.push_str("Pling");
    }
    if n % 5 == 0{
        text.push_str("Plang");
    }
    if n % 7 == 0{
        text.push_str("Plong");
    }
    if text.is_empty() {
        text.push_str(&n.to_string());
    } 
    text
}

fn main() {}