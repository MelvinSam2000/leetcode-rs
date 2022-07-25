/*
    1 - Two Sum
    Time: O(n)
    Space: O(n)
*/
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut diff = HashMap::new();
    for i in 0..nums.len() {
        let cmp = target - nums[i];
        if let Some(&x) = diff.get(&cmp) {
            return vec![x, i as i32];
        } else {
            diff.insert(nums[i], i as i32);
        }
    }
    unreachable!()
}

#[test]
fn t1() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
}
