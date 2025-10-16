use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();

    let word_lower = word.to_lowercase();
    let sorted_word = sorted_chars(&word_lower);

    for &candidate in possible_anagrams {
        let cand_lower = candidate.to_lowercase();

        // Skip identical words (case-insensitive, Unicode-safe)
        if cand_lower == word_lower {
            continue;
        }

        if sorted_chars(&cand_lower) == sorted_word {
            result.insert(candidate);
        }
    }

    result
}

fn sorted_chars(s: &str) -> Vec<char> {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_unstable();
    chars
}

fn main(){}