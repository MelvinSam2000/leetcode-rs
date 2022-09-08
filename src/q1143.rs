/*
    1143 - Longest Common Subsequence
    Time: O(m*n)
    Space: O(n)
*/
pub fn longest_common_subsequence(
    s1: String,
    s2: String,
) -> i32 {
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    let m = s1.len();
    let n = s2.len();
    let mut dp = [vec![0; n + 1], vec![0; n + 1]];
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            dp[0][j] = if s1[i] == s2[j] {
                1 + dp[1][j + 1]
            } else {
                dp[0][j + 1].max(dp[1][j])
            };
        }
        dp.swap(0, 1);
    }
    dp[1][0]
}

/*
    1143 - Longest Common Subsequence
    Time: O(n*m)
    Space: O(n*m)
    Note: Can be space optimized..
*/
pub fn longest_common_subsequence_v2(
    s1: String,
    s2: String,
) -> i32 {
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    let m = s1.len();
    let n = s2.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            dp[i][j] = if s1[i] == s2[j] {
                1 + dp[i + 1][j + 1]
            } else {
                dp[i][j + 1].max(dp[i + 1][j])
            };
        }
    }
    dp[0][0]
}

#[test]
fn t1() {
    let tcases = [
        ("abcde", "ace", 3),
        ("abc", "abc", 3),
        ("abc", "def", 0),
        ("bsbininm", "jmjkbkjkv", 1),
        ("aef", "efaaaaaa", 2),
    ];
    for (s1, s2, lcs) in tcases {
        assert_eq!(
            longest_common_subsequence(
                s1.to_owned(),
                s2.to_owned()
            ),
            lcs
        );
    }
}
