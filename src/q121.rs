/*
    121 - Best time to buy and sell stock
    Time: O(n)
    Space: O(1)
*/
pub fn max_profit(prices: Vec<i32>) -> i32 {
    use std::cmp::max;
    use std::cmp::min;
    let n = prices.len();

    if n <= 1 {
        return 0;
    }

    let mut min_price = prices[0];
    let mut max_profit = 0;
    for i in 1..n {
        min_price = min(min_price, prices[i]);
        max_profit = max(prices[i] - min_price, max_profit);
    }
    max_profit
}
