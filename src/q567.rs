/*
    567 - Permutation in String
    Time: O(m*n) (Amortized its O(m + n))
    Space: O(m)
    Note: Uses XOR rolling hash function, with bruteforce to check collisions
*/
pub fn check_inclusion(s1: String, s2: String) -> bool {
    use std::collections::HashMap;
    if s2.len() < s1.len() {
        return false;
    }
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    let m = s1.len();
    let n = s2.len();

    let h1 = s1.iter().fold(0, |res, &x| res ^ x);
    let mut h2 =
        s2.iter().take(m).fold(0, |res, &x| res ^ x);

    let (mut l, mut r) = (0, m);

    while r <= n {
        if h1 == h2 {
            let freq = (0..m).fold(
                HashMap::new(),
                |mut freq, i| {
                    freq.entry(s1[i])
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                    freq.entry(s2[l + i])
                        .and_modify(|c| *c -= 1)
                        .or_insert(-1);
                    freq
                },
            );
            if freq.values().all(|&c| c == 0) {
                return true;
            }
        }
        if r == n {
            break;
        }
        h2 ^= s2[l] ^ s2[r];
        l += 1;
        r += 1;
    }
    false
}
