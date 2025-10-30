pub fn reply(message: &str) -> &str {

    let trimmed = message.trim();
    let has_letters = trimmed.chars().any(|c| c.is_alphabetic());
    let is_yelling = has_letters && trimmed == trimmed.to_uppercase();
    let is_question = trimmed.ends_with('?');

    if is_question && is_yelling{
        "Calm down, I know what I'm doing!"
    }
        
    else if is_question{
        "Sure."
    }
    else if is_yelling{
        "Whoa, chill out!"
    }
    else if trimmed.is_empty() {
        "Fine. Be that way!"
    }
    else{
        "Whatever."
    }
}

fn main() {}