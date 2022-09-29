/*
    58 - Length of the last word
    Time: O(n)
    Space: O(1)
*/
pub fn length_of_last_word(s: String) -> i32 {
    s.split_whitespace()
        .last()
        .map(|w| w.len() as i32)
        .unwrap_or(0)
}
