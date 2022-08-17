/*
    9 - Palindrome Number
    Time: O(logn)
    Space: O(1)
*/
pub fn is_palindrome(mut x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let n = (x as f64).log10() as i32 + 1;
    for i in 0..n / 2 {
        let l = x / 10i32.pow((n - 2 * i - 1) as u32);
        let r = x % 10;
        if l != r {
            return false;
        }
        x -= l * 10i32.pow((n - 2 * i - 1) as u32);
        x /= 10;
    }
    true
}

#[test]
fn t1() {
    let tcases = [(121, true), (1001, true), (9999, true)];
    for (n, res) in tcases {
        assert_eq!(is_palindrome(n), res);
    }
}
