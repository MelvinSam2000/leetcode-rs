/*
    424 - Longest Repeating Character Replacement
    Time: O(n)
    Space: O(n)
*/
pub fn character_replacement(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let k = k as usize;

    let mut count = vec![0; 26];
    let mut res = 0;
    let mut l = 0;

    for r in 0..n {
        count[(s[r] - b'A') as usize] += 1;
        while (r - l + 1) - count.iter().max().unwrap() > k {
            count[(s[l] - b'A') as usize] -= 1;
            l += 1;
        }
        res = res.max(r - l + 1);
    }
    res as i32
}
