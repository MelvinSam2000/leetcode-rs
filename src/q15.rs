/*
    15 - 3Sum
    Time: O(n^2)
    Space: O(1)
    Note: Basically sort, then do TwoSum II.
*/
pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::cmp::Ordering;
    let n = nums.len();
    let mut triplets = vec![];
    nums.sort();

    for i in 0..n - 2 {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        let mut l = i + 1;
        let mut r = n - 1;
        while l < r {
            match (nums[i] + nums[l] + nums[r]).cmp(&0) {
                Ordering::Equal => {
                    triplets.push(vec![
                        nums[i], nums[l], nums[r],
                    ]);
                    l += 1;
                    while l < r && nums[l - 1] == nums[l] {
                        l += 1;
                    }
                }
                Ordering::Less => {
                    l += 1;
                }
                Ordering::Greater => {
                    r -= 1;
                }
            }
        }
    }
    triplets
}

#[test]
fn t1() {
    let tcases = [(
        vec![-1, 0, 1, 2, -1, -4],
        vec![vec![-1, -1, 2], vec![-1, 0, 1]],
    )];
    for (nums, triplets) in tcases {
        assert_eq!(three_sum(nums), triplets);
    }
}
