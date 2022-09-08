/*
    494 - Target Sum (Optimized)
    Time: O(n*s)
    Space: O(s)
*/
pub fn find_target_sum_ways(
    nums: Vec<i32>,
    target: i32,
) -> i32 {
    let n = nums.len();
    let s = nums.iter().sum::<i32>() as usize;
    let target = target.unsigned_abs() as usize;
    if target > s {
        return 0;
    }
    let mut dp = [vec![0; s + 1], vec![0; s + 1]];
    dp[1][nums[0] as usize] = 1 + (nums[0] == 0) as i32;

    for i in 1..n {
        for j in 0..=s {
            let l = (j as i32 - nums[i]).unsigned_abs()
                as usize;
            let r = j + nums[i] as usize;
            dp[0][j] = if l <= s { dp[1][l] } else { 0 };
            dp[0][j] += if r <= s { dp[1][r] } else { 0 };
        }
        dp.swap(0, 1);
    }
    dp[1][target]
}

/*
    494 - Target Sum
    Time: O(n*s)
    Space: O(n*s)
    Note: Space can be optimized to O(s)
*/
pub fn find_target_sum_ways_v2(
    nums: Vec<i32>,
    target: i32,
) -> i32 {
    let n = nums.len();
    let s = nums.iter().sum::<i32>() as usize;
    let target = target.unsigned_abs() as usize;
    if target > s {
        return 0;
    }
    let mut dp = vec![vec![0; s + 1]; n];
    dp[0][nums[0] as usize] = 1 + (nums[0] == 0) as i32;

    for i in 1..n {
        for j in 0..=s {
            let l = (j as i32 - nums[i]).unsigned_abs()
                as usize;
            let r = j + nums[i] as usize;
            dp[i][j] +=
                if l <= s { dp[i - 1][l] } else { 0 };
            dp[i][j] +=
                if r <= s { dp[i - 1][r] } else { 0 };
        }
    }
    dp[n - 1][target]
}
