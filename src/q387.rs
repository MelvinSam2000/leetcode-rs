/*
    387 - First Unique Char in String
    Time: O(n)
    Space: O(n)
*/
pub fn first_uniq_char(s: String) -> i32 {
    use std::collections::HashMap;
    let s = s.as_bytes();
    let mut count = HashMap::new();
    for ch in s {
        if let Some(&x) = count.get(&ch) {
            count.insert(ch, x + 1);
        } else {
            count.insert(ch, 0);
        }
    }
    for (i, ch) in s.iter().enumerate() {
        if count.get(ch) == Some(&0) {
            return i as i32;
        }
    }
    -1
}
