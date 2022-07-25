/*
    217 - Contains Duplicate
*/
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    use std::collections::HashSet;

    let mut visited = HashSet::new();
    for elem in nums.into_iter() {
        if visited.contains(&elem) {
            return true;
        } else {
            visited.insert(elem);
        }
    }
    false
}
