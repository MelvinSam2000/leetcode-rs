/*
    125 - Valid Palindrome
    Time: O(n)
    Space: O(n)
    Note: Could try to optimize space by not creating a second copy of string without alphanumeric characters.
    In that case space would be O(1)
*/

pub fn is_palindrome(s: String) -> bool {
    let s = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase();
    let s = s.as_bytes();
    let n = s.len();
    if n <= 1 {
        return true;
    }
    for i in 0..(n / 2 + 1) {
        if s[i] != s[n - i - 1] {
            return false;
        }
    }
    true
}

#[test]
fn t1() {
    let tcases = [
        ("A man, a plan, a canal: Panama", true),
        ("race a car", false),
        (" ", true),
        ("  ", true),
    ];
    for (param, out) in tcases.into_iter() {
        assert_eq!(is_palindrome(param.to_owned()), out);
    }
}
