/*
    41 - First Missing Positive
    Time: O(n)
    Space: O(1)
*/
pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();

    // remove negative numbers
    for i in 0..n {
        if nums[i] < 0 {
            nums[i] = 0;
        }
    }

    // convert to hashset from 1 to n - 1
    for i in 0..n {
        if (1..=n as i32).contains(&nums[i].abs()) {
            let j = nums[i].unsigned_abs() as usize - 1;
            nums[j] = if nums[j] == 0 {
                -(j as i32 + 1)
            } else {
                -nums[j].abs()
            };
        }
    }

    // get first missing positive
    for i in 0..n {
        if nums[i] >= 0 {
            return i as i32 + 1;
        }
    }
    n as i32 + 1
}
