/*
    263 - Ugly Number
    Time: O(logn)
    Space: O(1)
*/
pub fn is_ugly(mut n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    let factors = [2, 3, 5];
    let mut done = false;
    while !done {
        done = true;
        for factor in factors {
            if n % factor == 0 {
                n /= factor;
                done = false;
            }
        }
    }
    n == 1
}
