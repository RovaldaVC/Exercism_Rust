pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();

    // Replace separators
    let cleaned = phrase.replace('-', " ").replace('_', " ");

    // Insert a space before uppercase letters that follow lowercase ones (camel case)
    let mut adjusted = String::new();
    let mut prev_lower = false;
    for c in cleaned.chars() {
        if prev_lower && c.is_uppercase() {
            adjusted.push(' ');
        }
        adjusted.push(c);
        prev_lower = c.is_lowercase();
    }

    // Split and collect first letters
    for word in adjusted.split_whitespace() {
        if let Some(first) = word.chars().next() {
            acronym.push(first);
        }
    }

    acronym.to_uppercase()
}

fn main(){}