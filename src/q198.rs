/*
    198 - House Robber
    Time: O(n)
    Space: O(1)
*/
pub fn rob(nums: Vec<i32>) -> i32 {
    use std::cmp::max;
    let n = nums.len();
    if n == 1 {
        return nums[0];
    }
    if n == 2 {
        return max(nums[0], nums[1]);
    }

    let mut dp_2 = nums[0];
    let mut dp_1 = max(nums[0], nums[1]);
    let mut dp = 0;
    for i in 2..n {
        dp = max(dp_1, nums[i] + dp_2);
        dp_2 = dp_1;
        dp_1 = dp;
    }
    dp
}

#[test]
fn t1() {
    assert_eq!(rob(vec![1, 2, 3, 1]), 4);
}
