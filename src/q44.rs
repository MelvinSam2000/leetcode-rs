/*
    44 - Wildcard Matching
    Time: O(m*n)
    Space: O(n)
*/
pub fn is_match(s: String, p: String) -> bool {
    let s = s + "$";
    let p = p + "$";
    let s = s.as_bytes();
    let p = p.as_bytes();
    let m = s.len();
    let n = p.len();
    let mut dp = [vec![false; n], vec![false; n]];
    dp[1][n - 1] = true;
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            dp[0][j] = if p[j] == b'?' || s[i] == p[j] {
                p[j] == b'$'
                    || (i < m - 1
                        && j < n - 1
                        && dp[1][j + 1])
            } else if p[j] == b'*' {
                (i < m - 1 && dp[1][j])
                    || (j < n - 1 && dp[0][j + 1])
            } else {
                false
            };
        }
        dp.swap(0, 1);
    }
    dp[1][0]
}

/*
    44 - Wildcard Matching (Unoptimized)
    Time: O(m*n)
    Space: O(m*n) (Can be optimized to O(n))
*/
pub fn is_match_v2(s: String, p: String) -> bool {
    let s = s + "$";
    let p = p + "$";
    let s = s.as_bytes();
    let p = p.as_bytes();
    let m = s.len();
    let n = p.len();
    let mut dp = vec![vec![false; n]; m];
    dp[m - 1][n - 1] = true;
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            dp[i][j] = if p[j] == b'?' || s[i] == p[j] {
                p[j] == b'$'
                    || (i < m - 1
                        && j < n - 1
                        && dp[i + 1][j + 1])
            } else if p[j] == b'*' {
                (i < m - 1 && dp[i + 1][j])
                    || (j < n - 1 && dp[i][j + 1])
            } else {
                false
            };
        }
    }
    dp[0][0]
}
