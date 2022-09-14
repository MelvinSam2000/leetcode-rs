/*
    123 - Best time to buy and sell stock III
    Time: O(n)
    Space: O(1)
*/
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let n = prices.len();

    // dp[i, j, k] where i is prices index,
    // j is available txns, and k is 0 for buy, 1 for sell
    let mut dp = [[[0; 2]; 3]; 2];
    dp[1][2] = [0, prices[n - 1]];
    dp[1][1] = [0, prices[n - 1]];
    dp[1][0] = [0, 0];

    for i in (0..n - 1).rev() {
        for j in 1..=2 {
            dp[0][j][0] =
                dp[1][j][0].max(dp[1][j][1] - prices[i]);
            dp[0][j][1] = dp[1][j][1]
                .max(dp[1][j - 1][0] + prices[i]);
        }
        dp.swap(0, 1);
    }

    dp[1][2][0]
}

/*
    123 - Best time to buy and sell stock III
    Time: O(n)
    Space: O(n) (Can be optimized to O(1))
*/
pub fn max_profit_v2(prices: Vec<i32>) -> i32 {
    let n = prices.len();

    // dp[i, j, k] where i is prices index,
    // j is available txns, and k is 0 for buy, 1 for sell
    let mut dp = vec![[[0; 2]; 3]; n];
    dp[n - 1][2] = [0, prices[n - 1]];
    dp[n - 1][1] = [0, prices[n - 1]];
    dp[n - 1][0] = [0, 0];

    for i in (0..n - 1).rev() {
        for j in 1..=2 {
            dp[i][j][0] = dp[i + 1][j][0]
                .max(dp[i + 1][j][1] - prices[i]);
            dp[i][j][1] = dp[i + 1][j][1]
                .max(dp[i + 1][j - 1][0] + prices[i]);
        }
    }

    dp[0][2][0]
}
