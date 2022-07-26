/*
    45 - Jump Game II
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
            covered = max(max_covered, i + nums[i] as usize);
            jumps += 1;
            if covered >= n - 1 {
                return jumps;
            }
        } else {
            max_covered = max(max_covered, i + nums[i] as usize)
        }
    }
    jumps
}
