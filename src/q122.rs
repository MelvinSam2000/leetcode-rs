/*
    122 - Best Time to buy and sell stock II
    Time: O(n)
    Space: O(1)
*/
pub fn max_profit(prices: Vec<i32>) -> i32 {
    prices
        .windows(2)
        .filter(|v| v[0] < v[1])
        .map(|v| v[1] - v[0])
        .sum()
}
