/*
    376 - Wiggle Subsequence
    Time: O(n)
    Space: O(1)
*/
pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
    let n = nums.len();

    if n <= 1 {
        return n as i32;
    }

    let mut count = 1;
    let (mut i, mut j) = (1, 0);

    while i < n {
        if nums[j] < nums[i] {
            count += 1;
            while i < n - 1 && nums[i] <= nums[i + 1] {
                i += 1;
            }
        } else if nums[j] > nums[i] {
            count += 1;
            while i < n - 1 && nums[i] >= nums[i + 1] {
                i += 1;
            }
        }
        j = i;
        i += 1;
    }

    return count;
}
