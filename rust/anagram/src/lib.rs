use std::collections::{HashMap, HashSet};

fn to_count_map(word: &str) -> HashMap<char, i32> {
    let mut map = HashMap::new();
    for c in word.chars() {
        *map.entry(c).or_default() += 1;
    }
    map
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let base = to_count_map(&word);
    possible_anagrams
        .iter()
        .filter(|target| {
            let target = target.to_lowercase();
            word != target && base == to_count_map(&target)
        })
        .cloned()
        .collect()
}
