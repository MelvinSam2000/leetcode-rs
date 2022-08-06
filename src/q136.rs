/*
    136 - Single Number
    Time: O(n)
    Space: O(1)
*/
pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |x, y| x ^ y)
}
