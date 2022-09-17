/*
    256 - Paint House
    Time: O(n)
    Space: O(1)
*/
pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
    use std::i32::MAX;
    let n = costs.len();
    let mut dp = [[0; 3]; 2];
    for i in (0..n).rev() {
        for j in 0..3 {
            dp[0][j] = MAX;
            for k in 0..3 {
                if j != k {
                    dp[0][j] = dp[0][j]
                        .min(costs[i][k] + dp[1][k]);
                }
            }
        }
        dp.swap(0, 1);
    }
    dp[1].iter().min().cloned().unwrap()
}

/*
    256 - Paint House
    Time: O(n)
    Space: O(n) (unoptimized)
*/
pub fn min_cost_v2(costs: Vec<Vec<i32>>) -> i32 {
    use std::i32::MAX;
    let n = costs.len();
    let mut dp = vec![[0; 3]; n + 1];
    for i in (0..n).rev() {
        for j in 0..3 {
            dp[i][j] = MAX;
            for k in 0..3 {
                if j != k {
                    dp[i][j] = dp[i][j]
                        .min(costs[i][k] + dp[i + 1][k]);
                }
            }
        }
    }
    dp[0].iter().min().cloned().unwrap()
}
