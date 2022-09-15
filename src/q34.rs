/*
    34 - Find First and Last Position of Element in Sorted Array
    Time: O(logn)
    Space: O(1)
*/
#[allow(clippy::collapsible_else_if)]
pub fn search_range(
    nums: Vec<i32>,
    target: i32,
) -> Vec<i32> {
    fn binary_search<const UPPER: bool>(
        nums: &[i32],
        target: i32,
    ) -> Option<i32> {
        use std::cmp::Ordering;

        let (mut l, mut r) = (0, nums.len() as i32 - 1);
        while l <= r {
            let m = ((l + r) / 2) as usize;
            match nums[m].cmp(&target) {
                Ordering::Equal => {
                    if !UPPER {
                        if m == 0 || nums[m - 1] != nums[m]
                        {
                            return Some(m as i32);
                        } else {
                            r = m as i32 - 1;
                        }
                    } else {
                        if m == nums.len() - 1
                            || nums[m + 1] != nums[m]
                        {
                            return Some(m as i32);
                        } else {
                            l = m as i32 + 1;
                        }
                    }
                }
                Ordering::Less => {
                    l = m as i32 + 1;
                }
                Ordering::Greater => {
                    r = m as i32 - 1;
                }
            }
        }
        None
    }

    vec![
        binary_search::<false>(&nums, target).unwrap_or(-1),
        binary_search::<true>(&nums, target).unwrap_or(-1),
    ]
}
