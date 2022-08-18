/*
    231 - Power of Two
    Time: O(1)
    Space: O(1)
*/
pub fn is_power_of_two(n: i32) -> bool {
    let mut n = n as i64;
    if n == 0 {
        return false;
    }
    if n < 0 {
        n *= -n;
    }
    let mut found = false;
    for _ in 0..64 {
        if n & 1 != 0 {
            if found {
                return false;
            } else {
                found = true;
            }
        }
        n >>= 1;
    }
    true
}
