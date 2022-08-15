/*
    50 - Pow x n
    Time: O(logn)
    Space: O(1)
*/
pub fn my_pow(mut x: f64, n: i32) -> f64 {
    let mut n = n as i64;
    if n < 0 {
        n = -n;
        x = 1.0 / x;
    }
    let mut res = 1.0;
    while n > 0 {
        if n & 1 != 0 {
            res *= x;
        }
        x *= x;
        n >>= 1;
    }
    res
}
