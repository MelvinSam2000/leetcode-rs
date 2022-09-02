/*
    279 - Perfect Squares
    Time: O(n^2)
    Space: O(n)
*/
pub fn num_squares(n: i32) -> i32 {
    const K: usize = 95;
    static SQUARES: [usize; K] = {
        let mut out = [0; K];
        let mut i = 0;
        while i < K {
            out[i] = i.pow(2) as usize;
            i += 1;
        }
        out
    };

    let n = n as usize;
    let mut dp = (1..=n + 1).collect::<Vec<_>>();

    for &square in &SQUARES {
        if square > n {
            break;
        }
        dp[square] = 1;
    }

    for i in 1..=n {
        for &square in &SQUARES {
            if square > i {
                break;
            }
            dp[i] = dp[i].min(dp[square] + dp[i - square]);
        }
    }
    dp[n] as i32
}
