use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str>{
    let mut result = HashSet::new();
    for possible_word in possible_anagrams {
        if word.to_lowercase() == *possible_word.to_lowercase() || word.len() != possible_word.len() {
            continue;
        }
        let mut vec_word: Vec<char> = word.to_lowercase().chars().collect();
        let mut vec_possible_word: Vec<char> = possible_word.to_lowercase().chars().collect();
        
        vec_word.sort_unstable();
        vec_possible_word.sort_unstable();
        let matching = vec_word.iter().zip(vec_possible_word.iter()).filter(|&(a, b)| a == b).count();
        if matching == vec_word.len() {
            result.insert(*possible_word);
        }
    }
    result
}
