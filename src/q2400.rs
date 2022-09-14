/*
    2400 - Number of Ways to Reach a Position After Exactly k Steps
    Time: O(k^2)
    Space: O(k)
*/

pub fn number_of_ways(start: i32, end: i32, k: i32) -> i32 {
    use std::collections::HashMap;

    const MOD: i32 = 1000000007;

    let mut map: HashMap<i32, i32> = HashMap::from_iter(
        (start - k..=start + k).map(|i| (i, 0)),
    );
    map.insert(start, 1);

    let mut dp = [map.clone(), map];

    for _ in 0..=k {
        for j in start - k..=start + k {
            dp[1].insert(
                j,
                (dp[0]
                    .get(&(j - 1))
                    .cloned()
                    .unwrap_or_default()
                    + dp[0]
                        .get(&(j + 1))
                        .cloned()
                        .unwrap_or_default())
                    % MOD,
            );
        }
        dp.swap(0, 1);
    }

    dp[1].get(&end).cloned().unwrap_or_default()
}
