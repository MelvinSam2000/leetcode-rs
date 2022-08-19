/*
    680 - Valid Palindrome II
    Time: O(n)
    Space: O(1)
*/
pub fn valid_palindrome(s: String) -> bool {
    let s = s.as_bytes();
    let mut l = 0;
    let mut r = s.len() - 1;
    while l < r {
        if s[l] != s[r] {
            return palindrome(&s[l + 1..=r]) || palindrome(&s[l..r]);
        }
        l += 1;
        r -= 1;
    }
    true
}

fn palindrome(s: &[u8]) -> bool {
    let mut l = 0;
    let mut r = s.len() - 1;
    while l < r {
        if s[l] != s[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }
    true
}
