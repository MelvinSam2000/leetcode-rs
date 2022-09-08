/*
    45 - Jump Game II (Optimized)
    Time: O(n)
    Space: O(1)
*/
pub fn jump(nums: Vec<i32>) -> i32 {
    use std::cmp::max;
    let n = nums.len();
    if n == 1 {
        return 0;
    }
    let mut jumps = 0;
    let mut covered = 0;
    let mut max_covered = 0;
    for i in 0..n {
        if i == covered {
            covered =
                max(max_covered, i + nums[i] as usize);
            jumps += 1;
            if covered >= n - 1 {
                return jumps;
            }
        } else {
            max_covered =
                max(max_covered, i + nums[i] as usize)
        }
    }
    jumps
}

/*
    45 - Jump Game II
    Time: O(s) where s = sum of all elems in nums
    Space: O(n)
    Note: DP solution I came up with. Not optimal
*/
pub fn jump_v2(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut dp = vec![std::i32::MAX - 1; n];
    dp[0] = 0;
    for i in 0..n - 1 {
        for j in 1..=(nums[i] as usize).min(n - i - 1) {
            dp[i + j] = dp[i + j].min(1 + dp[i]);
        }
    }
    dp[n - 1]
}
