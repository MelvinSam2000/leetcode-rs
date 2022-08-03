use std::collections::HashSet;

/*
    46 - Permutations
    Time: O(n!)
    Space: O(n!)
*/
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut out = vec![];
    let mut available: HashSet<i32> = HashSet::from_iter(nums);
    let mut permutation = vec![];
    helper(&mut available, &mut permutation, &mut out);
    out
}

fn helper(available: &mut HashSet<i32>, permutation: &mut Vec<i32>, out: &mut Vec<Vec<i32>>) {
    if available.is_empty() {
        out.push(permutation.clone());
        return;
    }

    for elem in available.clone().into_iter() {
        available.remove(&elem);
        permutation.push(elem);
        helper(available, permutation, out);
        permutation.pop();
        available.insert(elem);
    }
}
