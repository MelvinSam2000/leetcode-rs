/*
    338 - Count bits
    Time: O(nlogn)
    Space: O(1)
*/
pub fn count_bits(n: i32) -> Vec<i32> {
    let n = n as usize + 1;
    let mut out = vec![0; n];
    for i in 0..n {
        let mut j = i as i32;
        while j > 0 {
            out[i] += j & 1;
            j >>= 1;
        }
    }
    out
}
