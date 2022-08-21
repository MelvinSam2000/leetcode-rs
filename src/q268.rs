/*
    268 - Missing Number
    Time: O(n)
    Space: O(1)
*/
pub fn missing_number(nums: Vec<i32>) -> i32 {
    let n = nums.len() as i32;
    let s: i32 = nums.into_iter().sum();
    n * (n + 1) / 2 - s
}
