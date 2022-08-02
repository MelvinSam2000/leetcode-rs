/*
    287 - Find Duplicate
    Time: O(n)
    Space: O(1)
*/
pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut slow = 0;
    let mut fast = 0;
    loop {
        slow = nums[slow] as usize;
        fast = nums[nums[fast] as usize] as usize;
        if slow == fast {
            break;
        }
    }
    let mut slow2 = 0;
    loop {
        slow = nums[slow] as usize;
        slow2 = nums[slow2] as usize;
        if slow == slow2 {
            break;
        }
    }
    slow2 as i32
}

#[test]
fn t1() {
    let tcases = [
        (vec![1, 3, 4, 2, 2], 2),
        (vec![3, 1, 3, 4, 2], 3),
        (vec![1, 2, 1], 1),
        (vec![1, 1], 1),
    ];
    for (nums, repeated) in tcases {
        assert_eq!(find_duplicate(nums), repeated);
    }
}
