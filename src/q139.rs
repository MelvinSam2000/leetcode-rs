/*
    139 - Word Break
    n is len of s, m is total num of chars in word_dict
    Time: O(n*m)
    Space: O(n)
*/
pub fn word_break(
    s: String,
    word_dict: Vec<String>,
) -> bool {
    let s = s.as_bytes();
    let n = s.len();
    let mut dp = vec![false; n + 1];
    dp[n] = true;
    'a: for i in (0..n).rev() {
        for word in word_dict.iter() {
            let w = word.as_bytes();
            let mut j = 0;
            while i + j < n
                && j < w.len()
                && s[i + j] == w[j]
            {
                j += 1;
            }
            if j == w.len() && dp[i + j] {
                dp[i] = true;
                continue 'a;
            }
        }
        dp[i] = false;
    }
    dp[0]
}
