/*
    35 - Search Insert Position
    Time: O(logn)
    Space: O(1)
*/
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    use std::cmp::Ordering;

    let mut l = 0;
    let mut r = nums.len() as i32 - 1;
    while l <= r {
        let m = (l + r) / 2;
        match nums[m as usize].cmp(&target) {
            Ordering::Equal => return m,
            Ordering::Less => l = m + 1,
            Ordering::Greater => r = m - 1,
        }
    }
    r + 1
}
