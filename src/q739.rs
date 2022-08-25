/*
    739 - Daily Temperatures
    Time: O(n)
    Space: O(n)
    Note: Monotonic Stack problem
*/
pub fn daily_temperatures(temp: Vec<i32>) -> Vec<i32> {
    let n = temp.len();
    let mut monostack = vec![];
    monostack.push((0, temp[0]));
    let mut out = vec![0; n];
    for i in 1..n {
        while let Some(&(j, elem)) = monostack.last() {
            if elem < temp[i] {
                monostack.pop();
                out[j] = (i - j) as i32;
            } else {
                break;
            }
        }
        monostack.push((i, temp[i]));
    }
    out
}
