/*
    1512 - Number of good pairs
    Time: O(n)
    Space: O(n)
*/
pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    nums.into_iter()
        .fold(HashMap::new(), |mut freq, num| {
            freq.entry(num)
                .and_modify(|c| *c += 1)
                .or_insert(1);
            freq
        })
        .values()
        .map(|n| n * (n - 1) / 2)
        .sum()
}
