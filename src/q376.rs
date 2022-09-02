/*
    376 - Wiggle Subsequence
    Time: O(n)
    Space: O(1)
*/
pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
    use std::cmp::Ordering;
    let n = nums.len();

    if n <= 1 {
        return n as i32;
    }

    let mut count = 1;
    let (mut i, mut j) = (1, 0);

    while i < n {
        match nums[j].cmp(&nums[i]) {
            Ordering::Less => {
                count += 1;
                while i < n - 1 && nums[i] <= nums[i + 1] {
                    i += 1;
                }
            }
            Ordering::Greater => {
                count += 1;
                while i < n - 1 && nums[i] >= nums[i + 1] {
                    i += 1;
                }
            }
            Ordering::Equal => {}
        }
        j = i;
        i += 1;
    }
    count
}
