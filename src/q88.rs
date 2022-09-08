/*
    88 - Merge 2 sorted arrays
    Time: O(m + n)
    Space: O(m + n)
    Note: Apparently can be done in O(1) space...
*/
pub fn merge(
    nums1: &mut Vec<i32>,
    m: i32,
    nums2: &mut Vec<i32>,
    n: i32,
) {
    let (m, n) = (m as usize, n as usize);
    let (mut i, mut j) = (0, 0);
    let mut res = vec![0; m + n];
    loop {
        match (i < m, j < n) {
            (true, true) => {
                if nums1[i] <= nums2[j] {
                    res[i + j] = nums1[i];
                    i += 1;
                } else {
                    res[i + j] = nums2[j];
                    j += 1;
                }
            }
            (true, false) => {
                res[i + j] = nums1[i];
                i += 1;
            }
            (false, true) => {
                res[i + j] = nums2[j];
                j += 1;
            }
            (false, false) => break,
        }
    }
    *nums1 = res;
}
