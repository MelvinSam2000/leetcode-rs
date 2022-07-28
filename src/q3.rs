/*
    3 - Longest Substring without repeating characters
    Time: O(n)
    Space: O(n)
*/
pub fn length_of_longest_substring(s: String) -> i32 {
    use std::collections::HashSet;
    use std::cmp::max;

    let chars = s.as_bytes();
    let n = chars.len();
    let mut visited = HashSet::new();
    let mut j = 0;
    let mut res = 0;
    for i in 0..n {
        while j < n && !visited.contains(&chars[j]) {
            visited.insert(chars[j]);
            j += 1;
            res = max(res, j - i);
        }
        visited.remove(&chars[i]);
    }
    res as i32
}


#[test]
fn t1() {
    let tcases = [
        ("abcabcbb", 3),
        ("bbbbb", 1),
        ("pwwkew", 3)
    ];
    for (param, expected) in tcases {
        assert_eq!(length_of_longest_substring(param.to_owned()), expected);
    }
}