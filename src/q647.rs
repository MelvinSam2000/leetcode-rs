/*
    647 - Palindromic Substrings
    Time: O(n^2)
    Space: O(n^2)
    Note: Space can probably be optimized to O(n)
*/
pub fn count_substrings(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();

    let mut p = vec![vec![false; n]; n];
    let mut dp = vec![1; n];

    // init p (p[i][j] tracks whether s[i..=j] is palindrome)
    for i in (0..n).rev() {
        for j in 0..n {
            let next = if i < n - 1 && j > 0 && i + 1 <= j - 1 {
                p[i + 1][j - 1]
            } else {
                true
            };
            p[i][j] = s[i] == s[j] && next;
        }
    }

    // init dp (dp[i] counts # of palindromic substings in s[i..])
    for i in (0..n - 1).rev() {
        dp[i] = dp[i + 1];
        for j in i..n {
            dp[i] += p[i][j] as i32;
        }
    }
    dp[0]
}
