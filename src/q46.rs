use std::collections::HashSet;

/*
    46 - Permutations
    Time: O(n!)
    Space: O(n!)
*/
pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let n = nums.len();
    let mut out = vec![];
    helper(&mut nums, 0, n, &mut out);
    out
}

fn helper(arr: &mut Vec<i32>, i: usize, n: usize, out: &mut Vec<Vec<i32>>) {
    if i == n - 1 {
        out.push(arr.clone());
    } else {
        helper(arr, i + 1, n, out);
        for j in i + 1..n {
            arr.swap(i, j);
            helper(arr, i + 1, n, out);
            arr.swap(i, j);
        }
    }
}
