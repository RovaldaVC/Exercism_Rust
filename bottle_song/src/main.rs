pub fn recite(start_bottles: u32, take_down: u32) -> String {
    if take_down == 0 || start_bottles == 0 {
        return String::new();
    }

    fn word(n: i32, capitalize: bool) -> String {
        let s = match n {
            0 => "no".to_string(),
            1 => "one".to_string(),
            2 => "two".to_string(),
            3 => "three".to_string(),
            4 => "four".to_string(),
            5 => "five".to_string(),
            6 => "six".to_string(),
            7 => "seven".to_string(),
            8 => "eight".to_string(),
            9 => "nine".to_string(),
            10 => "ten".to_string(),
            _ => n.to_string(),
        };
        if capitalize {
            let mut c = s.chars();
            match c.next() {
                Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
                None => s,
            }
        } else {
            s
        }
    }

    let mut verses = Vec::new();
    let mut current = start_bottles as i32;
    let end = current - take_down as i32 + 1;

    while current >= end {
        let first = format!(
            "{0} green bottle{1} hanging on the wall,",
            word(current, true),
            if current == 1 { "" } else { "s" }
        );
        let second = format!(
            "{0} green bottle{1} hanging on the wall,",
            word(current, true),
            if current == 1 { "" } else { "s" }
        );
        let third = format!(
            "And if one green bottle should accidentally fall,"
        );

        let next_count = current - 1;
        let fourth = if next_count == 0 {
            format!("There'll be no green bottles hanging on the wall.")
        } else {
            format!(
                "There'll be {} green bottle{} hanging on the wall.",
                word(next_count, false),
                if next_count == 1 { "" } else { "s" }
            )
        };

        verses.push(format!("{}\n{}\n{}\n{}", first, second, third, fourth));
        current -= 1;
    }

    verses.join("\n\n")
}

fn main(){}