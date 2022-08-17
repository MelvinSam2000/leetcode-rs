/*
    344 - Reverse String
    Time: O(n)
    Space: O(1)
*/
pub fn reverse_string(s: &mut Vec<char>) {
    let n = s.len();
    for i in 0..n / 2 {
        s.swap(i, n - i - 1);
    }
}
