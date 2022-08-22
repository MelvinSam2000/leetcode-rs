/*
    347 - K Most frequent Elements
    Time: O(n)
    Space: O(n)
*/
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let n = nums.len() + 1;
    let k = k as usize;

    let mut freq = HashMap::new();
    for &num in nums.iter() {
        freq.entry(num).and_modify(|c| *c += 1).or_insert(1);
    }
    let mut m = vec![vec![]; n];
    for (elem, count) in freq {
        m[count].push(elem);
    }

    let mut out = vec![];
    let mut j = n - 1;
    while out.len() < k {
        for &elem in m[j].iter() {
            out.push(elem);
            if out.len() == k {
                break;
            }
        }
        if j == 0 {
            break;
        }
        j -= 1;
    }
    out
}
