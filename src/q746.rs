/*
    746 - Min Cost Climbing Stairs
    Time: O(n)
    Space: O(n)
*/
pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    use std::cmp::min;
    let n = cost.len();
    let mut dp = vec![0; n];
    dp[0] = cost[0];
    dp[1] = cost[1];
    for i in 2..n {
        dp[i] = cost[i] + min(dp[i - 1], dp[i - 2]);
    }
    min(dp[n - 1], dp[n - 2])
}

#[test]
fn t1() {
    assert_eq!(min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    assert_eq!(
        min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
        6
    );
}
