/*
    172 - Factorial Trailing Zeroes
    Time: O(logn)
    Space: O(1)
    Note: Number theory nonsense... Count # of 5 prime factors in n!
*/
pub fn trailing_zeroes(n: i32) -> i32 {
    let mut out = 0;
    let mut i = 5;
    while i <= n {
        out += n / i;
        i *= 5;
    }
    out
}
