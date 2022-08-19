/*
    309 - Best time to buy/sell stock with cooldown
    Time: O(n)
    Space: O(n)
    Note: Can be optimized to use O(1) space
*/
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let n = prices.len();
    // buy: 0, sell: 1
    let mut dp = vec![[0, 0]; n];
    dp[n - 1][1] = prices[n - 1];

    for i in (0..n - 1).rev() {
        dp[i][0] = dp[i + 1][0].max(dp[i + 1][1] - prices[i]);
        dp[i][1] = dp[i + 1][1].max(if i < n - 2 { dp[i + 2][0] } else { 0 } + prices[i]);
    }
    dp[0][0]
}

pub fn max_profit_top_down(prices: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    fn helper(i: usize, buying: bool, dp: &mut HashMap<(usize, bool), i32>, prices: &[i32]) -> i32 {
        if i >= prices.len() {
            return 0;
        }
        if let Some(&val) = dp.get(&(i, buying)) {
            return val;
        }
        let cooldown = helper(i + 1, buying, dp, prices);
        if buying {
            let buy = helper(i + 1, false, dp, prices) - prices[i];
            let out = cooldown.max(buy);
            dp.insert((i, buying), out);
            out
        } else {
            let sell = helper(i + 2, true, dp, prices) + prices[i];
            let out = cooldown.max(sell);
            dp.insert((i, buying), out);
            out
        }
    }

    let mut dp = HashMap::new();
    let out = helper(0, true, &mut dp, &prices);
    out
}

#[test]
fn t1() {
    let tcases = [(vec![1, 7, 2, 4], 7)];
    for (nums, expected) in tcases {
        assert_eq!(max_profit(nums), expected);
    }
}
