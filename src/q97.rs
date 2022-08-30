/*
    97 - Interleaving String (Optimized)
    Time: O(m*n)
    Space: O(n)
*/
pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    if s1.len() + s2.len() != s3.len() {
        return false;
    }
    let s1 = s1 + "$";
    let s1 = s1.as_bytes();
    let s2 = s2 + "$";
    let s2 = s2.as_bytes();
    let s3 = s3.as_bytes();
    let m = s1.len();
    let n = s2.len();

    let mut dp = [vec![false; n], vec![false; n]];
    dp[0][n - 1] = true;
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if i != m - 1 || j != n - 1 {
                dp[0][j] = match (
                    i != m - 1 && s1[i] == s3[i + j],
                    j != n - 1 && s2[j] == s3[i + j],
                ) {
                    (true, true) => dp[1][j] || dp[0][j + 1],
                    (true, false) => dp[1][j],
                    (false, true) => dp[0][j + 1],
                    (false, false) => false,
                }
            }
        }
        dp.swap(0, 1);
    }
    dp[1][0]
}

/*
    97 - Interleaving String
    Time: O(m*n)
    Space: O(m*n) (Can be optimized to O(n))
*/
pub fn is_interleave_v2(s1: String, s2: String, s3: String) -> bool {
    if s1.len() + s2.len() != s3.len() {
        return false;
    }
    let s1 = s1 + "$";
    let s1 = s1.as_bytes();
    let s2 = s2 + "$";
    let s2 = s2.as_bytes();
    let s3 = s3.as_bytes();
    let m = s1.len();
    let n = s2.len();

    let mut dp = vec![vec![false; n]; m];
    dp[m - 1][n - 1] = true;
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if i != m - 1 || j != n - 1 {
                dp[i][j] = match (
                    i != m - 1 && s1[i] == s3[i + j],
                    j != n - 1 && s2[j] == s3[i + j],
                ) {
                    (true, true) => dp[i + 1][j] || dp[i][j + 1],
                    (true, false) => dp[i + 1][j],
                    (false, true) => dp[i][j + 1],
                    (false, false) => false,
                }
            }
        }
    }
    dp[0][0]
}
