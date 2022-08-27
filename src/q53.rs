/*
    53 - Maximum Subarray
    Time: O(n)
    Space: O(1)
    Note: This is Kadane's algorithm
*/
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut dp = nums[0];
    let mut res = nums[0];
    for elem in nums.into_iter().skip(1) {
        dp = elem.max(dp + elem);
        res = res.max(dp);
    }
    res
}
