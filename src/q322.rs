/*
    322 - Coin Change
    Time: O(n)
    Space: O(n)
*/
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    use std::cmp::min;

    let mut dp = vec![amount + 1; amount as usize + 1];
    dp[0] = 0;
    for i in 0..=amount as usize {
        for c in &coins {
            let c = *c as usize;
            if i >= c {
                dp[i] = min(dp[i], 1 + dp[i - c]);
            }
        }
    }
    if dp[amount as usize] == amount + 1 {
        return -1;
    }
    dp[amount as usize]
}

#[test]
fn t1() {
    let tcases = [(vec![1, 2, 5], 11, 3), (vec![2], 3, -1)];
    for (coins, amount, res) in tcases {
        assert_eq!(coin_change(coins, amount), res)
    }
}
