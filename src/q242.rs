/*
    242 - Valid Anagram
    Time: O(n)
    Space: O(n)
*/
pub fn is_anagram(s: String, t: String) -> bool {
    use std::collections::HashMap;

    if s.len() != t.len() {
        return false;
    }
    let mut counts = HashMap::new();
    for (ch1, ch2) in s.chars().zip(t.chars()) {
        if let Some(count) = counts.get_mut(&ch1) {
            *count += 1;
        } else {
            counts.insert(ch1, 1);
        }
        if let Some(count) = counts.get_mut(&ch2) {
            *count -= 1;
        } else {
            counts.insert(ch2, -1);
        }
    }
    counts.values().all(|&count| count == 0)
}

#[test]
fn t1() {
    assert!(is_anagram("anagram".to_owned(), "nagaram".to_owned()));
}
