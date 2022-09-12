/*
    14 - Longest Common Prefix
    Time: O(m*n)
    Space: O(n)
*/
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let strs = strs
        .iter()
        .map(|s| s.as_bytes())
        .collect::<Vec<_>>();
    let n = strs.iter().map(|s| s.len()).min().unwrap();
    let mut res = "".to_owned();
    for i in 0..n {
        if strs.windows(2).all(|v| v[0][i] == v[1][i]) {
            res += &String::from_utf8_lossy(&[strs[0][i]]);
        } else {
            break;
        }
    }
    res
}
