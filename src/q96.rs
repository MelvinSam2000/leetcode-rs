/*
    96 - Number of unique BSTs (Optimized)
    Time: O(n)
    Space: O(1)
    Note: Computes nth catalan number using formula.
    May overflow with large input n.
*/
pub fn num_trees(n: i32) -> i32 {
    let n = n as i128;
    let a = (2..=n).fold(1, |prod, k| prod * (n + k));
    let b = (2..=n).product::<i128>();
    (a / b) as i32
}

/*
    96 - Number of unique BSTs
    Time: O(n)
    Space: O(n)
    Note: Nth catalan number problem.
    Can be optimized to use O(1) with the
    direct formula, but that solution is unlikely to be in an interview.
    Full formula may not work due to floating point errors as well.
*/
pub fn num_trees_v2(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    dp[1] = 1;
    for i in 2..=n {
        dp[i] = (0..i).map(|j| dp[j] * dp[i - j - 1]).sum();
    }
    dp[n]
}

/*
    96 - Number of unique BSTs (Compile time computation)
    Time: O(1)
    Space: O(1)
    Note: Nth catalan number problem.
    Can be optimized to use O(1) with the
    direct formula, but that solution is unlikely to be in an interview.
    Full formula may not work due to floating point errors as well.
*/
pub fn num_trees_ebic(n: i32) -> i32 {
    const N: usize = 20;
    static CATALAN: [i32; N] = {
        let mut dp = [0; N];
        dp[0] = 1;
        dp[1] = 1;
        let mut i = 2;
        while i < N {
            let mut j = 0;
            while j < i {
                dp[i] += dp[j] * dp[i - j - 1];
                j += 1;
            }
            i += 1;
        }
        dp
    };

    CATALAN[n as usize]
}
