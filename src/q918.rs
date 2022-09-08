/*
    918 - Maximum Sum Circular Subarray
    Time: O(n)
    Space: O(1)
    Note: MAX SUM CIRC = TOTAL SUM - MIN SUM KADANE
*/
pub fn max_subarray_sum_circular(
    mut nums: Vec<i32>,
) -> i32 {
    let x = kadane(&nums);
    let mut s = 0;
    for elem in nums.iter_mut() {
        s += *elem;
        *elem *= -1;
    }
    let y = kadane(&nums);
    let z = s + y;
    if z == 0 {
        x
    } else {
        x.max(z)
    }
}

fn kadane(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut a = nums[0];
    let mut b = nums[0];
    for i in 1..n {
        a = nums[i].max(nums[i] + a);
        b = b.max(a);
    }
    b
}

#[test]
fn t1() {
    let tcases = [(vec![1, -2, 3, -2], 3)];
    for (nums, expected) in tcases {
        assert_eq!(
            max_subarray_sum_circular(nums),
            expected
        );
    }
}
