/*
    7 - Reverse Integer
    Time: O(logn)
    Space: O(1)
*/
pub fn reverse(x: i32) -> i32 {
    let mut x = x as i64;
    let neg = x < 0;
    if neg {
        x *= -1;
    }
    let mut out = 0;
    while x > 0 {
        out *= 10;
        out += x % 10;
        x /= 10;
    }
    if out > std::i32::MAX as i64 {
        return 0;
    }
    if neg {
        out *= -1;
    }
    out as i32
}
