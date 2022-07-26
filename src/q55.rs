/*
    55 - Jump Game
    Time: O(n)
    Space: O(1)
    Note: Has 2 solutions, DP and greedy, greedy being most efficient
*/
pub fn can_jump(nums: Vec<i32>) -> bool {
    let n = nums.len();
    let mut goal = n - 1;
    for i in (0..n).rev() {
        if i + nums[i] as usize >= goal {
            goal = i;
        }
    }
    goal == 0
}

#[test]
fn t1() {
    let tcases = [(vec![2, 3, 1, 1, 4], true), (vec![3, 2, 1, 0, 4], false)];
    for (param, res) in tcases {
        assert_eq!(can_jump(param), res);
    }
}
