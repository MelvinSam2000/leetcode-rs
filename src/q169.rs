/*
    169 - Majority Element
    Time: O(n)
    Space: O(1)
    Note: Uses Boyer Moore Voting Algorithm
*/
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut count = 0;
    for elem in nums {
        if count == 0 {
            res = elem;
            count += 1;
        } else if elem == res {
            count += 1;
        } else {
            count -= 1;
        }
    }
    res
}
