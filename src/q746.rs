/*
    746 - Min Cost Climbing Stairs
    Time: O(n)
    Space: O(1)
*/
pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    use std::cmp::min;
    let n = cost.len();

    if n == 2 {
        return min(cost[0], cost[1]);
    }

    let mut dp_2 = cost[0];
    let mut dp_1 = cost[1];
    let mut dp = 0;

    for i in 2..n {
        dp = cost[i] + min(dp_1, dp_2);
        dp_2 = dp_1;
        dp_1 = dp;
    }
    min(dp, dp_2)
}

// Previous version (Space O(n))
fn _min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
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
