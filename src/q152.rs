/*
    152 - Maximum Product Subarray
    Time: O(n)
    Space: O(1)
*/
pub fn max_product(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut dp = [[nums[0]; 2], [nums[0]; 2]];
    let mut res = nums[0];
    for i in 1..n {
        dp[0][0] = nums[i].min(nums[i] * dp[1][0]).min(nums[i] * dp[1][1]);
        dp[0][1] = nums[i].max(nums[i] * dp[1][0]).max(nums[i] * dp[1][1]);
        res = res.max(dp[0][1]);
        dp[1] = dp[0];
    }
    res
}

/*
    152 - Maximum Product Subarray (Unoptimized/Readable)
    Time: O(n)
    Space: O(n)
*/
pub fn max_product_v2(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut dp = [vec![0; n], vec![0; n]];
    dp[0][0] = nums[0];
    dp[1][0] = nums[0];
    let mut res = nums[0];
    for i in 1..n {
        dp[0][i] = nums[i]
            .min(nums[i] * dp[0][i - 1])
            .min(nums[i] * dp[1][i - 1]);
        dp[1][i] = nums[i]
            .max(nums[i] * dp[0][i - 1])
            .max(nums[i] * dp[1][i - 1]);
        res = res.max(dp[1][i]);
    }
    res
}
