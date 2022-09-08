/*
    740 - Delete and Earn (optimized)
    Time: O(k) (k = max element of nums)
    Space: O(n)
    Note: Slightly Optimized. DP part is O(1) space but there is O(n) for freq counter
*/
pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    let k = nums.iter().max().cloned().unwrap() as usize;
    let freq = nums.into_iter().fold(
        vec![0; k + 1],
        |mut freq, num| {
            freq[num as usize] += 1;
            freq
        },
    );

    let mut dp = [freq[1], freq[1], 0];
    for i in 2..=k {
        dp[0] = dp[1].max((i as i32) * freq[i] + dp[2]);
        dp[2] = dp[1];
        dp[1] = dp[0];
    }
    dp[0]
}

/*
    740 - Delete and Earn
    Time: O(n)
    Space: O(k), k = max element of nums
    Note: Could optimize dp vec to not use O(k) memory, but total space would remain O(k) due to count vec.
*/
pub fn delete_and_earn_v2(nums: Vec<i32>) -> i32 {
    use std::cmp::max;
    let n = nums.len();
    if n == 1 {
        return nums[0];
    }
    let k: usize =
        nums.iter().max().cloned().unwrap() as usize + 1;
    let mut count = vec![0; k];
    for i in 0..n {
        count[nums[i] as usize] += 1;
    }
    let mut dp = vec![0; k];
    dp[1] = count[1];
    for i in 2..k {
        dp[i] = max(
            dp[i - 1],
            (i as i32) * count[i] + dp[i - 2],
        );
    }
    dp[k - 1]
}

#[test]
fn t1() {
    let tests = [
        (vec![3, 4, 2], 6),
        (vec![2, 2, 3, 3, 3, 4], 9),
        (vec![3, 1], 4),
    ];
    for (val_in, val_out) in tests {
        assert_eq!(delete_and_earn(val_in), val_out);
    }
}
