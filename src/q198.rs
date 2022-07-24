use std::cmp::max;

/*
    198 - House Robber
    Time: O(n)
    Space: O(n)
*/
pub fn rob(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 1 {
        return nums[0];
    }

    let mut dp = vec![0; n];
    dp[0] = nums[0];
    dp[1] = max(nums[0], nums[1]);
    for i in 2..n {
        dp[i] = max(dp[i - 1], nums[i] + dp[i - 2])
    }
    dp[n - 1]
}

#[test]
fn t1() {
    assert_eq!(rob(vec![1, 2, 3, 1]), 4);
}
