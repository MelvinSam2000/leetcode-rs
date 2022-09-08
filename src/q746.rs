/*
    746 - Min Cost Climbing Stairs (optimized)
    Time: O(n)
    Space: O(1)
*/
pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let n = cost.len();
    let mut dp = [0, cost[1], cost[0]];
    for i in 2..n {
        dp[0] = cost[i] + dp[1].min(dp[2]);
        dp[2] = dp[1];
        dp[1] = dp[0];
    }
    dp[1].min(dp[2])
}

/*
    746 - Min Cost Climbing Stairs
    Time: O(n)
    Space: O(n)
    Note: Can be optimized to O(1) space
*/
pub fn min_cost_climbing_stairs_v2(cost: Vec<i32>) -> i32 {
    let n = cost.len();
    let mut dp = vec![0; n];
    dp[0] = cost[0];
    dp[1] = cost[1];
    for i in 2..n {
        dp[i] = cost[i] + dp[i - 1].min(dp[i - 2]);
    }
    dp[n - 1].min(dp[n - 2])
}

#[test]
fn t1() {
    assert_eq!(
        min_cost_climbing_stairs(vec![10, 15, 20]),
        15
    );
    assert_eq!(
        min_cost_climbing_stairs(vec![
            1, 100, 1, 1, 1, 100, 1, 1, 100, 1
        ]),
        6
    );
}
