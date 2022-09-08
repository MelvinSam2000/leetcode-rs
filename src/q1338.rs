/*
    1338 - Reduce Array Size by Half
    Time: O(n*logn)
    Space: O(n)
*/
pub fn min_set_size(arr: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let n = arr.len();

    let mut freq = HashMap::new();
    for elem in arr {
        let count =
            freq.get(&elem).map(|&x| x + 1).unwrap_or(1);
        freq.insert(elem, count);
    }
    let mut counts = freq.values().collect::<Vec<_>>();
    counts.sort();

    let mut out = 0;
    let mut total = n;
    for &count in counts.iter().rev() {
        total -= count;
        out += 1;
        if total <= n / 2 {
            return out;
        }
    }
    out
}
