/*
    1 - Two Sum
    Time: O(n)
    Space: O(n)
*/
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut m = HashMap::new();
    for (j, &elem) in nums.iter().enumerate() {
        if let Some(&i) = m.get(&elem) {
            return vec![i as i32, j as i32];
        } else {
            m.insert(target - elem, j);
        }
    }
    unreachable!()
}

#[test]
fn t1() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
}
