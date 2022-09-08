/*
    239 - Sliding Window Maximum
    Time: O(n)
    Space: O(n)
*/
pub fn max_sliding_window(
    nums: Vec<i32>,
    k: i32,
) -> Vec<i32> {
    use std::collections::VecDeque;
    let mut res = vec![];
    let mut d = VecDeque::new();
    for (i, &n) in nums.iter().enumerate() {
        while d
            .back()
            .map(|&j| nums[j] <= n)
            .unwrap_or(false)
        {
            d.pop_back();
        }
        d.push_back(i);
        if d.front()
            .map(|&j| j == i - k as usize)
            .unwrap_or(false)
        {
            d.pop_front();
        }
        if i >= k as usize - 1 {
            if let Some(&j) = d.front() {
                res.push(nums[j]);
            }
        }
    }
    res
}
