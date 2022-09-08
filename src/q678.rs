/*
    678 - Valid Parenthesis String
    Time: O(n^2)
    Space: O(n)
    Note: This uses Bottom Up DP, but Top Down could be better
*/
pub fn check_valid_string(s: String) -> bool {
    let s = s.as_bytes();
    let n = s.len();
    let mut dp = [vec![false; n + 1], vec![false; n + 1]];
    dp[1][0] = true;
    for i in (0..n).rev() {
        for j in 0..n {
            dp[0][j] = match s[i] {
                b'(' => dp[1][j + 1],
                b')' => j > 0 && dp[1][j - 1],
                b'*' => {
                    (j > 0 && dp[1][j - 1])
                        || dp[1][j]
                        || dp[1][j + 1]
                }
                _ => unreachable!(),
            }
        }
        dp.swap(0, 1);
    }
    dp[1][0]
}

/*
    678 - Valid Parenthesis String (Unoptimized)
    Time: O(n^2)
    Space: O(n^2)
*/
pub fn check_valid_string_v2(s: String) -> bool {
    let s = s.as_bytes();
    let n = s.len();
    let mut dp = vec![vec![false; n + 1]; n + 1];
    dp[n][0] = true;
    for i in (0..n).rev() {
        for j in 0..n {
            dp[i][j] = match s[i] {
                b'(' => dp[i + 1][j + 1],
                b')' => j > 0 && dp[i + 1][j - 1],
                b'*' => {
                    (j > 0 && dp[i + 1][j - 1])
                        || dp[i + 1][j]
                        || dp[i + 1][j + 1]
                }
                _ => unreachable!(),
            }
        }
    }
    dp[0][0]
}
