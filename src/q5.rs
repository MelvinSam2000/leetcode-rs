/*
    5 - Longest Palindromic Substring
    Time: O(n^2)
    Space: O(1)
    Note: Not sure if its O(1) or O(k) since it depends whether all the contents are automatically copied from previous String
*/
pub fn longest_palindrome(s: String) -> String {
    let v = s.as_bytes();
    let n = v.len();
    let mut lp = "";
    for i in 0..n {
        // handle odd and even palindromes
        for k in 0..=(i != n - 1) as usize {
            let (mut l, mut r) = (i, i + k);
            while v[l] == v[r] {
                if lp.len() < s[l..=r].len() {
                    lp = &s[l..=r];
                }
                if l == 0 || r == n - 1 {
                    break;
                }
                l -= 1;
                r += 1;
            }
        }
    }
    lp.to_owned()
}

#[test]
fn t1() {
    let tcases = [("babad", "bab"), ("cbbd", "bb")];
    for (param, out) in tcases {
        assert_eq!(
            longest_palindrome(param.to_owned()),
            out.to_owned()
        );
    }
}
