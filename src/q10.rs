/*
    10 - Regular Expression Matching (optimized)
    m: len of s, n: len of p
    Time: O(m*n)
    Space: O(m + n)
*/
pub fn is_match(s: String, p: String) -> bool {
    let mut s = s.as_bytes().to_vec();
    let mut p = p.as_bytes().to_vec();
    s.push(b'$');
    p.push(b'$');

    let m = s.len();
    let n = p.len();
    let mut dp = [vec![false; n], vec![false; n]];
    dp[0][n - 1] = true;

    for i in (0..m).rev() {
        for j in (0..n - 1).rev() {
            dp[0][j] = if p[j + 1] == b'*' {
                (j < n - 2 && dp[0][j + 2])
                    || (i < m - 1
                        && (s[i] == p[j] || p[j] == b'.')
                        && dp[1][j])
            } else if i < m - 1
                && (s[i] == p[j] || p[j] == b'.')
            {
                dp[1][j + 1]
            } else {
                false
            };
        }
        if i == m - 2 {
            dp[1][n - 1] = false;
        }
        dp.swap(0, 1);
    }
    dp[1][0]
}

/*
    10 - Regular Expression Matching
    m: len of s, n: len of p
    Time: O(m*n)
    Space: O(m*n)
    Note: Sacrificing readability, space can be optimized to O(n + m)
*/
pub fn is_match_v2(s: String, p: String) -> bool {
    let mut s = s.as_bytes().to_vec();
    let mut p = p.as_bytes().to_vec();
    s.push(b'$');
    p.push(b'$');

    let m = s.len();
    let n = p.len();
    let mut dp = vec![vec![false; n]; m];
    dp[m - 1][n - 1] = true;

    for i in (0..m).rev() {
        for j in (0..n - 1).rev() {
            dp[i][j] = if p[j + 1] == b'*' {
                (j < n - 2 && dp[i][j + 2])
                    || (i < m - 1
                        && (s[i] == p[j] || p[j] == b'.')
                        && dp[i + 1][j])
            } else if i < m - 1
                && (s[i] == p[j] || p[j] == b'.')
            {
                dp[i + 1][j + 1]
            } else {
                false
            };
        }
    }
    dp[0][0]
}

#[test]
fn t1() {
    let tcases = [
        ("aa", "a*", true),
        ("aab", "c*a*b", true),
        ("mississippi", "mis*is*p*.", false),
    ];

    for (s, p, res) in tcases {
        assert_eq!(
            is_match(s.to_owned(), p.to_owned()),
            res
        );
    }
}
