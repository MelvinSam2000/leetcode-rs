/*
    219 - Contains Duplicate II
    Time: O(n)
    Space: O(n)
*/
pub fn contains_nearby_duplicate(
    nums: Vec<i32>,
    k: i32,
) -> bool {
    use std::collections::HashMap;
    let mut visited = HashMap::new();
    for i in 0..nums.len() {
        if let Some(j) = visited.get(&nums[i]) {
            if i - j <= k as usize {
                return true;
            }
        }
        visited.insert(nums[i], i);
    }
    false
}
