/*
    62 - Unique Paths (Compile time computation)
    Time: O(1)
    Space: O(1)
*/
pub fn unique_paths(m: i32, n: i32) -> i32 {
    const M: usize = 100;
    const N: usize = 100;
    static PATHS: [[i32; N]; M] = {
        let mut paths = [[0; N]; M];
        paths[0][0] = 1;
        let mut i = 0;
        while i < M {
            let mut j = 0;
            while j < N {
                if i > 0 {
                    let x = (paths[i][j] as i32)
                        .wrapping_add(paths[i - 1][j]);
                    paths[i][j] = if paths[i - 1][j] > x {
                        std::i32::MAX
                    } else {
                        x
                    };
                }
                if j > 0 {
                    let x = (paths[i][j] as i32)
                        .wrapping_add(paths[i][j - 1]);
                    paths[i][j] = if paths[i][j - 1] > x {
                        std::i32::MAX
                    } else {
                        x
                    };
                }
                j += 1;
            }
            i += 1;
        }
        paths
    };

    let (m, n) = (m as usize, n as usize);
    PATHS[m - 1][n - 1]
}

/*
    62 - Unique Paths
    Time: O(n*m)
    Space: O(n)
*/
pub fn unique_paths_v1(m: i32, n: i32) -> i32 {
    let n = n as usize;
    let mut dp = [vec![0; n], vec![0; n]];
    dp[0][0] = 1;
    dp[1][0] = 1;

    for _ in 0..m {
        for j in 1..n {
            dp[0][j] = dp[1][j] + dp[0][j - 1];
        }
        dp.swap(0, 1);
    }
    dp[1][n - 1]
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
