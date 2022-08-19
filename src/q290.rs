/*
    290 - Word Pattern
    Time: O(n)
    Space: O(n)
*/
pub fn word_pattern(pattern: String, s: String) -> bool {
    use std::collections::HashMap;

    let s = s.split(' ').collect::<Vec<_>>();
    let pattern = pattern.chars().collect::<Vec<_>>();
    if pattern.len() != s.len() {
        return false;
    }

    let mut fwd = HashMap::new();
    let mut back = HashMap::new();

    for (ch, word) in pattern.iter().zip(s.iter()) {
        match fwd.get(&ch) {
            Some(w) => {
                if w != &word {
                    return false;
                }
            }
            None => {
                if back.contains_key(&word) {
                    return false;
                }
                fwd.insert(ch, word);
                back.insert(word, ch);
            }
        }
    }
    true
}
