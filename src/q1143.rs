/*
    1143 - Longest Common Subsequence
    Time: O(n*m)
    Space: O(m)
    Note: This is the space optimized, but less readable version
*/
pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    use std::mem::swap;
    let s1 = text1.as_bytes();
    let s2 = text2.as_bytes();
    let m = s1.len() + 1;
    let n = s2.len() + 1;
    let mut dp1 = vec![0; m];
    let mut dp2 = vec![0; m];

    for j in 1..n {
        for i in 1..m {
            dp2[i] = if s1[i - 1] == s2[j - 1] {
                1 + dp1[i - 1]
            } else {
                dp2[i - 1].max(dp1[i])
            }
        }
        swap(&mut dp1, &mut dp2);
    }
    dp1[m - 1]
}

/*
    1143 - Longest Common Subsequence
    Time: O(n*m)
    Space: O(n*m)
    Note: Space can be optimized to O(min(n, m))
*/
pub fn longest_common_subsequence_v2(text1: String, text2: String) -> i32 {
    let s1 = text1.as_bytes();
    let s2 = text2.as_bytes();
    let m = s1.len() + 1;
    let n = s2.len() + 1;
    let mut dp = vec![vec![0; n]; m];

    for i in 1..m {
        for j in 1..n {
            dp[i][j] = if s1[i - 1] == s2[j - 1] {
                1 + dp[i - 1][j - 1]
            } else {
                dp[i - 1][j].max(dp[i][j - 1])
            }
        }
    }
    dp[m - 1][n - 1]
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
            longest_common_subsequence(s1.to_owned(), s2.to_owned()),
            lcs
        );
    }
}
