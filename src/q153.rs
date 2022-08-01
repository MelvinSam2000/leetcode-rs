/*
    153 - Minimum in Rotated Sorted Array
    Time: O(logn)
    Space: O(1)
*/
pub fn find_min(nums: Vec<i32>) -> i32 {
    nums[helper(&nums, 0, nums.len() - 1)]
}

fn helper(nums: &[i32], l: usize, r: usize) -> usize {
    let n = nums.len();
    if l >= r {
        return r;
    }
    if nums[l] < nums[r] {
        return l;
    }
    let m = (l + r) / 2;

    if m > 0 && nums[m - 1] > nums[m] {
        return m;
    } else if m < n - 1 && nums[m] > nums[m + 1] {
        return m + 1;
    }

    if nums[m] > nums[r] {
        helper(nums, m + 1, r)
    } else {
        helper(nums, l, m - 1)
    }
}
