/*
    704 - Binary Search
    Time: O(log(n))
    Space: O(1)
*/
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    binary_search(&nums, target)
}

fn binary_search(nums: &[i32], target: i32) -> i32 {
    use std::cmp::Ordering;
    let m = nums.len() >> 1;
    if nums.is_empty() || (nums.len() == 1 && nums[0] != target) {
        return -1;
    }

    match target.cmp(&nums[m]) {
        Ordering::Equal => m as i32,
        Ordering::Less => binary_search(&nums[..m], target),
        Ordering::Greater => match binary_search(&nums[m + 1..], target) {
            -1 => -1,
            i => m as i32 + i + 1,
        },
    }
}
