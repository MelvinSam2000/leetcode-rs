/*
    345 - Reverse Vowels
    Time: O(n)
    Space: O(n)
*/
pub fn reverse_vowels(s: String) -> String {
    use std::collections::HashSet;

    let n = s.len();
    let mut s = s.as_bytes().to_vec();
    let vowels: HashSet<u8> =
        HashSet::from_iter([b'a', b'e', b'i', b'o', b'u', b'A', b'E', b'I', b'O', b'U']);

    let mut l = 0;
    let mut r = n - 1;
    loop {
        while l <= r && !vowels.contains(&s[l]) {
            l += 1;
        }
        while l <= r && !vowels.contains(&s[r]) {
            r -= 1;
        }
        if l >= r {
            break;
        }
        s.swap(l, r);
        l += 1;
        r -= 1;
    }
    String::from_utf8_lossy(&s).to_string()
}
