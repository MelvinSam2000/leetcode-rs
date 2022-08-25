/*
    383 - Ransom Note
    Time: O(n + m)
    Space: O(n + m)
*/
pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    use std::collections::HashMap;

    let mut freq = HashMap::new();
    for ch in ransom_note.chars() {
        freq.entry(ch).and_modify(|c| *c -= 1).or_insert(-1);
    }
    for ch in magazine.chars() {
        freq.entry(ch).and_modify(|c| *c += 1).or_insert(1);
    }
    freq.values().all(|&c| c >= 0)
}
