/*
    39 - Combination Sum
    m - target, n - candidates.len()
    Time: O(n*m)
    Space: O(n*m)
    Note: Similar solution/approach to Coin Change II
*/
pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let m = target as usize + 1;
    let mut dp: Vec<Vec<Vec<i32>>> = vec![vec![]; m];
    dp[0].push(vec![]);

    for candidate in candidates {
        let candidate = candidate as usize;
        for i in candidate..m {
            for combination in dp[i - candidate].clone() {
                let mut new_comb = combination;
                new_comb.push(candidate as i32);
                dp[i].push(new_comb);
            }
        }
    }
    dp.remove(m - 1)
}

#[test]
fn t1() {
    let tcases = [(vec![2, 3, 6, 7], 7, vec![vec![2, 2, 3], vec![7]])];

    for (candidates, target, expected) in tcases {
        assert_eq!(combination_sum(candidates, target), expected);
    }
}
