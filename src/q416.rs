/*
    416 - Partition Equal Subset sum (Optimized)
    Time: O(n*s)
    Space: O(s)
*/
pub fn can_partition(nums: Vec<i32>) -> bool {
    let n = nums.len();
    let s = nums.iter().sum::<i32>() as usize;
    if s % 2 != 0 {
        return false;
    }

    let mut dp = [vec![false; s + 1], vec![false; s + 1]];
    dp[0][0] = true;
    dp[1][0] = true;
    dp[1][nums[0] as usize] = true;

    for i in 1..n {
        for j in 1..=s {
            let x = j as i32 - nums[i];
            dp[0][j] = dp[1][j] || (x >= 0 && dp[1][x as usize]);
            if dp[0][s / 2] {
                return true;
            }
        }
        dp.swap(0, 1);
    }
    dp[1][s / 2]
}

/*
    416 - Partition Equal Subset sum
    Time: O(n*s)
    Space: O(n*s)
    Note: Can be optimized to O(s) space
*/
pub fn can_partition_v2(nums: Vec<i32>) -> bool {
    let n = nums.len();
    let s = nums.iter().sum::<i32>() as usize;
    if s % 2 != 0 {
        return false;
    }

    let mut dp = vec![vec![false; s + 1]; n];
    dp[0][0] = true;
    dp[0][nums[0] as usize] = true;

    for i in 1..n {
        for j in 0..=s {
            let x = j as i32 - nums[i];
            dp[i][j] = dp[i - 1][j] || (x >= 0 && dp[i - 1][x as usize]);
            if dp[i][s / 2] {
                return true;
            }
        }
    }
    dp[n - 1][s / 2]
}

#[test]
fn t1() {
    let tcases = [
        (vec![2, 3, 5], true),
        (vec![1, 3, 5, 5, 14], true),
        (vec![1, 5, 10, 6], true),
        (vec![2, 2, 3, 5], false),
    ];
    for (nums, ans) in tcases {
        assert_eq!(can_partition(nums), ans);
    }
}
