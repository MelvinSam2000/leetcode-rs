/*
    215 - Kth Largest Element
    Time: O(n)
    Space: O(1) (O(n) with recursion)
    Note: Quick Select algorithm
*/
pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
    let i = nums.len() - k as usize;
    ith_order_stat(&mut nums, i)
}

fn ith_order_stat(nums: &mut [i32], i: usize) -> i32 {
    use std::cmp::Ordering;
    match nums.len() {
        1 => nums[0],
        _ => {
            let p = partition(nums);
            match i.cmp(&p) {
                Ordering::Equal => nums[i],
                Ordering::Less => {
                    ith_order_stat(&mut nums[..p], i)
                }
                Ordering::Greater => ith_order_stat(
                    &mut nums[p + 1..],
                    i - p - 1,
                ),
            }
        }
    }
}

fn partition(nums: &mut [i32]) -> usize {
    let n = nums.len();
    let mut i = 1;
    let mut j = n - 1;
    while i <= j {
        if nums[i] > nums[0] {
            nums.swap(i, j);
            j -= 1;
        } else {
            i += 1;
        }
    }
    nums.swap(0, j);
    j
}

/*
fn partition_valid(nums: &[i32], p: usize) -> bool {
    let l = nums[..p].iter().all(|&x| x <= nums[p]);
    let r = nums[p + 1..].iter().all(|&x| x >= nums[p]);
    l && r
}
*/

#[test]
fn t1() {
    let tcases = [(vec![3, 2, 1, 5, 6, 4], 2, 5)];
    for (arr, k, ans) in tcases {
        assert_eq!(find_kth_largest(arr, k), ans);
    }
}
