/*
    516 - Longest Palindromic Subsequence (Optimized)
    Time: O(n^2)
    Space: O(n)
*/
pub fn longest_palindrome_subseq(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut dp = vec![vec![0; n], vec![0; n]];
    for i in (0..n).rev() {
        for j in i..n {
            dp[0][j] = if i == j {
                1
            } else if s[i] == s[j] {
                if i + 1 == j {
                    2
                } else {
                    2 + dp[1][j - 1]
                }
            } else {
                dp[1][j].max(dp[0][j - 1])
            }
        }
        dp.swap(0, 1);
    }
    dp[1][n - 1]
}

/*
    516 - Longest Palindromic Subsequence
    Time: O(n^2)
    Space: O(n^2)
    Note: Space can be optimized to O(n)
*/
pub fn longest_palindrome_subseq_v2(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut dp = vec![vec![0; n]; n];
    for i in (0..n).rev() {
        for j in i..n {
            dp[i][j] = if i == j {
                1
            } else if s[i] == s[j] {
                if i + 1 == j {
                    2
                } else {
                    2 + dp[i + 1][j - 1]
                }
            } else {
                dp[i + 1][j].max(dp[i][j - 1])
            }
        }
    }
    dp[0][n - 1]
}
