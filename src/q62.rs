/*
    62 - Unique Paths
    Time: O(n*m)
    Space: O(min(n, m))
*/
pub fn unique_paths(m: i32, n: i32) -> i32 {
    use std::cmp::max;
    use std::cmp::min;
    use std::mem::swap;

    let (n, m) = (min(n, m) as usize, max(n, m));

    let mut dp1 = vec![0; n];
    let mut dp2 = vec![0; n];
    dp1[0] = 1;
    dp2[0] = 1;

    for _ in 0..m {
        for j in 1..n {
            dp2[j] = dp1[j] + dp2[j - 1];
        }
        swap(&mut dp1, &mut dp2);
    }
    dp1[n - 1]
}

/*
    (unoptimized) 62 - Unique Paths
    Time: O(n*m)
    Space: O(n*m)
*/
pub fn unique_paths_v2(m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let mut dp = vec![vec![0; n]; m];
    dp[0][0] = 1;
    for i in 0..m {
        for j in 0..n {
            if i == j && j == 0 {
                continue;
            }
            let x1 = if i != 0 { dp[i - 1][j] } else { 0 };
            let x2 = if j != 0 { dp[i][j - 1] } else { 0 };
            dp[i][j] = x1 + x2;
        }
    }
    dp[m - 1][n - 1]
}

#[test]
fn t1() {
    let tcases = [(3, 7, 28)];
    for (m, n, expected) in tcases {
        assert_eq!(unique_paths(m, n), expected);
    }
}
