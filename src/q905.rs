/*
    905 - Sort Array By Parity
    Time: O(n)
    Space: O(1)
*/
pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut l = 0;
    let mut r = n - 1;
    while l < r {
        while l < r && nums[l] % 2 == 0 {
            l += 1;
        }
        while l < r && nums[r] % 2 == 1 {
            r -= 1;
        }
        if l < r {
            nums.swap(l, r);
        }
    }
    nums
}
