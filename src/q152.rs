/*
    152 - Maximum Product Subarray
    Time: O(n)
    Space: O(n)
*/
pub fn max_product(nums: Vec<i32>) -> i32 {
    use std::cmp::max;
    use std::cmp::min;
    let n = nums.len();

    let mut acc_max = vec![0; n];
    let mut acc_min = vec![0; n];
    acc_max[0] = nums[0];
    acc_min[0] = nums[0];

    for i in 1..n {
        let pmax = acc_max[i - 1] * nums[i];
        let pmin = acc_min[i - 1] * nums[i];
        let xmax = max(pmax, pmin);
        let xmin = min(pmax, pmin);
        acc_max[i] = max(nums[i], xmax);
        acc_min[i] = min(nums[i], xmin);
    }
    acc_max.into_iter().max().unwrap()
}
