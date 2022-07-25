/*
    55 - Jump Game
    Time: O(n^2)
    Space: O(n)
*/
pub fn can_jump(nums: Vec<i32>) -> bool {
    use std::cmp::min;
    let n = nums.len();
    let mut dp = vec![false; n];
    dp[0] = true;
    for i in 0..n {
        for j in 1..=min(nums[i] as usize, n - i - 1) {
            dp[i + j] = dp[i];
        }
        if dp[n - 1] {
            return true;
        }
    }
    dp[n - 1]
}

#[test]
fn t1() {
    let tcases = [(vec![2, 3, 1, 1, 4], true), (vec![3, 2, 1, 0, 4], false)];
    for (param, res) in tcases {
        assert_eq!(can_jump(param), res);
    }
}
