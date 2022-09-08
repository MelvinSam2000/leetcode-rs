/*
    728 - Self dividing numbers
    Time: O(n*logn)
    Space: O(n)
*/
pub fn self_dividing_numbers(
    left: i32,
    right: i32,
) -> Vec<i32> {
    (left..=right)
        .filter(|&num| {
            let mut x = num;
            while x > 0 {
                let i = x % 10;
                if i == 0 || num % i != 0 {
                    return false;
                }
                x /= 10;
            }
            true
        })
        .collect::<Vec<_>>()
}
