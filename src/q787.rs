/*
    787 - Cheapest Flight within k stops
    Time: O(V*E)
    Space: O(V)
    Note: Slight variation of Bellman Ford algorithm.
*/
pub fn find_cheapest_price(
    n: i32,
    flights: Vec<Vec<i32>>,
    src: i32,
    dst: i32,
    k: i32,
) -> i32 {
    use std::i32::MAX;
    let n = n as usize;
    let src = src as usize;
    let dst = dst as usize;

    let mut dp = [vec![MAX; n], vec![MAX; n]];
    dp[0][src] = 0;
    dp[1][src] = 0;

    for _ in 0..=k {
        for edge in flights.iter() {
            let (u, v, w) = (
                edge[0] as usize,
                edge[1] as usize,
                edge[2],
            );
            dp[0][v] = dp[0][v].min(dp[1][v]).min(match dp
                [1][u]
            {
                MAX => MAX,
                _ => dp[1][u] + w,
            });
        }
        dp.swap(0, 1);
    }

    match dp[1][dst] {
        MAX => -1,
        x => x,
    }
}
