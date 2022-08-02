/*
    15 - 3Sum
    Time: O(?)
    Space: O(?)
*/
pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let n = nums.len();
    let mut triplets = vec![];
    nums.sort();
    let mut s = nums[0] + nums[1] + nums[2];
    for i in 0..n {
        if s == 0 {
            triplets.push(vec![nums[i], nums[i + 1], nums[i + 2]]);
        }
        s -= nums[i];
        s += nums[i + 3];
    }

    triplets
}
