/*
    33 - Search in Rotated Sorted Array
    Time: O(log(n))
    Space: O(1)
*/
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let index = find_min(&nums, 0, nums.len() - 1);
    let res = binary_search(&nums[..index], target);
    if res == -1 {
        match binary_search(&nums[index..], target) {
            -1 => -1,
            x => index as i32 + x,
        }
    } else {
        res
    }
}

fn find_min(nums: &[i32], l: usize, r: usize) -> usize {
    let n = nums.len();
    if l >= r {
        return r;
    }
    if nums[l] < nums[r] {
        return l;
    }
    let m = (l + r) / 2;

    if m > 0 && nums[m - 1] > nums[m] {
        return m;
    } else if m < n - 1 && nums[m] > nums[m + 1] {
        return m + 1;
    }

    if nums[m] > nums[r] {
        find_min(nums, m + 1, r)
    } else {
        find_min(nums, l, m - 1)
    }
}

fn binary_search(nums: &[i32], target: i32) -> i32 {
    use std::cmp::Ordering;
    let m = nums.len() >> 1;
    if nums.is_empty()
        || (nums.len() == 1 && nums[0] != target)
    {
        return -1;
    }

    match target.cmp(&nums[m]) {
        Ordering::Equal => m as i32,
        Ordering::Less => binary_search(&nums[..m], target),
        Ordering::Greater => {
            match binary_search(&nums[m + 1..], target) {
                -1 => -1,
                i => m as i32 + i + 1,
            }
        }
    }
}

#[test]
fn t1() {
    let tcases = [
        (vec![4, 5, 6, 7, 0, 1, 2], 0, 4),
        (vec![4, 5, 6, 7, 0, 1, 2], 3, -1),
        (vec![1], 0, -1),
        (vec![1, 3], 3, 1),
    ];
    for (i, (nums, target, index)) in
        tcases.into_iter().enumerate()
    {
        assert_eq!(
            search(nums, target),
            index,
            "Failed at test #{}",
            i
        );
    }
}
