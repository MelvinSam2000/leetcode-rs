/*
    342 - Power of Four
    Time: O(logn)
    Space: O(1)
*/
pub fn is_power_of_four(mut n: i32) -> bool {
    while n >= 4 {
        if n % 4 != 0 {
            return false;
        }
        n /= 4;
    }
    n == 1
}
