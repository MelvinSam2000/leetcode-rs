/*
    198 - House Robber
    Time: O(n)
    Space: O(1)
*/
pub fn rob(nums: Vec<i32>) -> i32 {
    match nums.len() {
        1 => nums[0],
        2 => nums[0].max(nums[1]),
        n => {
            let mut dp = [0, nums[0].max(nums[1]), nums[0]];
            for i in 2..n {
                dp[0] = dp[1].max(nums[i] + dp[2]);
                dp[2] = dp[1];
                dp[1] = dp[0];
            }
            dp[0]
        }
    }
}

//Previous version (Space O(n))
fn _rob(nums: Vec<i32>) -> i32 {
    use std::cmp::max;
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
