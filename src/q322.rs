/*
    322 - Coin Change
    Time: O(n*m)
    Space: O(n)
*/
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let n = amount as usize;
    let mut dp = vec![amount + 1; n + 1];
    dp[0] = 0;
    for i in 0..=n {
        for &coin in coins.iter() {
            let c = coin as usize;
            if i >= c {
                dp[i] = dp[i].min(1 + dp[i - c]);
            }
        }
    }
    if dp[n] == amount + 1 {
        -1
    } else {
        dp[n]
    }
}

#[test]
fn t1() {
    let tcases = [(vec![1, 2, 5], 11, 3), (vec![2], 3, -1)];
    for (coins, amount, res) in tcases {
        assert_eq!(coin_change(coins, amount), res)
    }
}
