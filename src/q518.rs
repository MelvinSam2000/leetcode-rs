/*
    518 - Coin Change II
    m = amount, n = len of coins vector
    Time: O(m*n)
    Space: O(m)
    Note: Unbounded Knapsack Problem.
*/
pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    let m = amount as usize + 1;

    let mut dp = vec![0; m];
    dp[0] = 1;

    for coin in coins {
        let coin = coin as usize;
        for i in coin..m {
            dp[i] += dp[i - coin];
        }
    }
    dp[m - 1]
}

/*
    (unoptimized) 518 - Coin Change II
    m = amount, n = len of coins vector
    Time: O(n*m)
    Space: O(n*m)
*/
pub fn change_v2(amount: i32, coins: Vec<i32>) -> i32 {
    let m = amount as usize + 1;
    let n = coins.len() + 1;
    let mut dp = vec![vec![0; n]; m];
    for j in 0..n {
        dp[0][j] = 1;
    }
    for i in 1..m {
        for j in (0..n - 1).rev() {
            dp[i][j] = dp[i][j + 1];
            if i as i32 - coins[j] >= 0 {
                dp[i][j] += dp[i - coins[j] as usize][j];
            }
        }
    }
    dp[m - 1][0]
}

#[test]
fn t1() {
    let tcases = [(5, vec![1, 2, 5], 4)];
    for (amount, coins, expected) in tcases {
        assert_eq!(change(amount, coins), expected);
    }
}
