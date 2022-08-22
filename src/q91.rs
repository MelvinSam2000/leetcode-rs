/*
    91 - Decode ways (Optimized)
    Time: O(n)
    Space: O(1)
*/
pub fn num_decodings(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len() + 1;
    let mut dp = [1; 3];

    for i in (0..n - 1).rev() {
        if s[i] == b'0' {
            dp[0] = 0;
        } else {
            dp[0] = dp[1];
            if i < n - 2 && (s[i] == b'1' || (s[i] == b'2' && (b'0'..=b'6').contains(&s[i + 1]))) {
                dp[0] += dp[2];
            }
        }
        dp[2] = dp[1];
        dp[1] = dp[0];
    }
    dp[0]
}

/*
    91 - Decode ways
    Time: O(n)
    Space: O(n)
    Note: Space can be optimized to O(1)
*/
pub fn num_decodings_v2(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len() + 1;
    let mut dp = vec![1; n];

    for i in (0..n - 1).rev() {
        if s[i] == b'0' {
            dp[i] = 0;
        } else {
            dp[i] = dp[i + 1];
            if i < n - 2 && (s[i] == b'1' || (s[i] == b'2' && (b'0'..=b'6').contains(&s[i + 1]))) {
                dp[i] += dp[i + 2];
            }
        }
    }
    dp[n - 1]
}
