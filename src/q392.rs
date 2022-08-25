/*
    392 - Is Subsequence
    Time: O(n)
    Space: O(1)
*/
pub fn is_subsequence(s: String, t: String) -> bool {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let m = s.len();
    let n = t.len();

    if s.is_empty() {
        return true;
    }

    let mut i = 0;
    for j in 0..n {
        if s[i] == t[j] {
            i += 1;
            if i == m {
                return true;
            }
        }
    }
    false
}
