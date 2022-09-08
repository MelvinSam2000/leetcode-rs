/*
    72 - Edit Distance (Optimized)
    Time: O(n*m)
    Space: O(n)
*/
pub fn min_distance(word1: String, word2: String) -> i32 {
    let word1 = word1.as_bytes();
    let word2 = word2.as_bytes();
    let m = word1.len() + 1;
    let n = word2.len() + 1;
    let mut dp = [vec![0; n], vec![0; n]];
    dp[0][n - 1] = 1;
    for j in 0..n {
        dp[1][j] = (n - j - 1) as i32;
    }
    for i in (0..m - 1).rev() {
        for j in (0..n - 1).rev() {
            dp[0][j] = if word1[i] == word2[j] {
                dp[1][j + 1]
            } else {
                1 + dp[1][j]
                    .min(dp[0][j + 1])
                    .min(dp[1][j + 1])
            };
        }
        dp[1][n - 1] += 2;
        dp.swap(0, 1);
    }

    dp[1][0]
}

/*
    72 - Edit Distance
    Time: O(n*m)
    Space: O(n*m)
    Note: Space can be optimized to O(m)
*/
pub fn min_distance_v2(
    word1: String,
    word2: String,
) -> i32 {
    let word1 = word1.as_bytes();
    let word2 = word2.as_bytes();
    let m = word1.len() + 1;
    let n = word2.len() + 1;
    let mut dp = vec![vec![0; n]; m];
    for i in 0..m {
        dp[i][n - 1] = m - i - 1;
    }
    for j in 0..n {
        dp[m - 1][j] = n - j - 1;
    }
    for i in (0..m - 1).rev() {
        for j in (0..n - 1).rev() {
            dp[i][j] = if word1[i] == word2[j] {
                dp[i + 1][j + 1]
            } else {
                1 + dp[i + 1][j]
                    .min(dp[i][j + 1])
                    .min(dp[i + 1][j + 1])
            };
        }
    }

    dp[0][0] as i32
}

pub fn print_2d_array<T: std::fmt::Display>(
    arr: &[Vec<T>],
) {
    let m = arr.len();
    let n = arr[0].len();
    for i in 0..m {
        for j in 0..n {
            print!("{} ", arr[i][j]);
        }
        println!();
    }
}

#[test]
fn t1() {
    let tcases = [
        ("horse", "ros", 3),
        ("abcc", "abbc", 1),
        ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "", 30),
        ("", "", 0),
    ];
    for (word1, word2, expected) in tcases {
        assert_eq!(
            min_distance(
                word1.to_owned(),
                word2.to_owned()
            ),
            expected
        );
    }
}
