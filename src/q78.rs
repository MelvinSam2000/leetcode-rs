/*
    78 - Subsets
    Time: O(n!)
    Space: O(n!)
*/
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    helper(&nums, &mut res, &mut vec![], 0);
    res
}

fn helper(nums: &[i32], res: &mut Vec<Vec<i32>>, subset: &mut Vec<i32>, i: usize) {
    res.push(subset.clone());
    for j in i..nums.len() {
        subset.push(nums[j]);
        helper(nums, res, subset, j + 1);
        subset.pop();
    }
}
