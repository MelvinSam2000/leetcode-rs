/*
    53 - Maximum Subarray
    Time: O(n)
    Space: O(1)
    Note: This is Kadane's algorithm
*/
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    use std::cmp::max;
    let n = nums.len();
    let mut local_max = nums[0];
    let mut global_max = nums[0];
    for i in 1..n {
        local_max = max(local_max + nums[i], nums[i]);
        global_max = max(local_max, global_max);
    }
    global_max
}
