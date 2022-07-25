/*
    213 - House Robber 2
    Time: O(n)
    Space: O(1)
*/
pub fn rob(nums: Vec<i32>) -> i32 {
    use std::cmp::max;
    let n = nums.len();
    if n == 1 {
        return nums[0];
    }
    let l = &nums[..n - 1];
    let r = &nums[1..];
    max(house_robber_1(l), house_robber_1(r))
}

fn house_robber_1(nums: &[i32]) -> i32 {
    use std::cmp::max;
    let n = nums.len();
    match n {
        0 => 0,
        1 => nums[0],
        2 => max(nums[0], nums[1]),
        _ => {
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
    }
}

#[test]
fn t1() {
    assert_eq!(rob(vec![2, 3, 2]), 3);
    assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(rob(vec![1, 2, 3]), 3);
}
