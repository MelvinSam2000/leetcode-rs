/*
    413 - Arithmetic Slices
    Time: O(n)
    Space: O(1)
*/
pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut res = 0;
    let mut m = 1;
    for i in 1..n - 1 {
        if nums[i + 1] - nums[i] == nums[i] - nums[i - 1] {
            res += m;
            m += 1;
        } else {
            m = 1;
        }
    }
    res
}
