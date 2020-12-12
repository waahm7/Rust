use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .cloned()
        .filter(|&str| is_anagram(word, str))
        .collect()
}

pub fn is_anagram(word1: &str, word2: &str) -> bool {
    let mut word1_chars: Vec<char> = word1.to_lowercase().chars().collect();
    let mut word2_chars: Vec<char> = word2.to_lowercase().chars().collect();
    if word1_chars == word2_chars {
        return false;
    }
    word1_chars.sort();
    word2_chars.sort();
    return word1_chars == word2_chars;
}
