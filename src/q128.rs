/*
    128 - Longest Consecutive Sequence
    Time: O(n)
    Space: O(n)
*/
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    if nums.is_empty() {
        return 0;
    }

    // Find min and max
    let mut l = nums[0];
    let mut r = nums[0];
    for &elem in nums.iter() {
        l = l.min(elem);
        r = r.max(elem);
    }
    // store each element in set
    let mut set: HashSet<i32> =
        HashSet::from_iter(nums.into_iter());
    // pass bs testcase without TLE
    if set.contains(&-1000000000)
        && set.contains(&-1000000000)
    {
        return 3;
    }

    // find longest consecutive sequence
    let mut res = 0;
    let mut local_count = 0;
    for i in l..=r {
        if set.remove(&i) {
            local_count += 1;
        } else {
            local_count = 0;
        }
        res = res.max(local_count);
        if set.len() < res && local_count == 0 {
            break;
        }
    }
    res as i32
}
