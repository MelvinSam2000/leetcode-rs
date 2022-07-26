/*
    300 - Longest Increasing Subsequence
    Time: O(n^2)
    Space: O(n)
*/
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    use std::cmp::max;
    let n = nums.len();
    let mut dp = vec![1; n];
    for i in (0..n).rev() {
        for j in i + 1..n {
            if nums[i] < nums[j] {
                dp[i] = max(dp[i], 1 + dp[j]);
            }
        }
    }
    *dp.iter().max().unwrap()
}
