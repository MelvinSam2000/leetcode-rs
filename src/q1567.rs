/*
    1567 - Maximum Length of Subarray with positive product
    Time: O(n)
    Space: O(1)
*/
pub fn get_max_len(nums: Vec<i32>) -> i32 {
    use std::cmp::Ordering;
    let mut res = 0;
    let mut pos = 0;
    let mut neg = 0;
    for elem in nums {
        match elem.cmp(&0) {
            Ordering::Equal => {
                pos = 0;
                neg = 0;
            }
            Ordering::Greater => {
                pos += 1;
                neg = if neg != 0 { neg + 1 } else { 0 };
            }
            Ordering::Less => {
                let old_pos = pos;
                pos = if neg != 0 { neg + 1 } else { 0 };
                neg = old_pos + 1;
            }
        }
        res = res.max(pos);
    }
    res
}

#[test]
fn t1() {
    let tcases = [(
        vec![
            5, -20, -20, -39, -5, 0, 0, 0, 36, -32, 0, -7, -10, -7, 21, 20, -12, -34, 26, 2,
        ],
        8,
    )];
    for (nums, count) in tcases {
        assert_eq!(get_max_len(nums), count);
    }
}
