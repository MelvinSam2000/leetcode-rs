/*
    118 - Pascals Triangle
    Time: O(n^2)
    Space: O(n^2)
*/
pub fn generate(n: i32) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut out = vec![vec![1]];
    for i in 1..n {
        out.push(vec![1]);
        for j in 1..i {
            let res = out[i - 1][j] + out[i - 1][j - 1];
            out[i].push(res);
        }
        out[i].push(1);
    }
    out
}
