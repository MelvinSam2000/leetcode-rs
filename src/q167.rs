/*
    167 - Two Sum II
    Time: O(n)
    Space: O(1)
*/
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    use std::cmp::Ordering;
    let mut l = 0;
    let mut r = numbers.len() - 1;
    while l < r {
        match (numbers[l] + numbers[r]).cmp(&target) {
            Ordering::Equal => {
                return vec![l as i32 + 1, r as i32 + 1];
            }
            Ordering::Less => {
                l += 1;
            }
            Ordering::Greater => {
                r -= 1;
            }
        }
    }
    unreachable!()
}
