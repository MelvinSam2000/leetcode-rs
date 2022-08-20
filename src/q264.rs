/*
    264 - Ugly Number II
    Time: O(n)
    Space: O(n)
*/
pub fn nth_ugly_number(n: i32) -> i32 {
    let n = n as usize;
    let mut ugly_nums = vec![1; n];
    let mut pos_2 = 0;
    let mut pos_3 = 0;
    let mut pos_5 = 0;

    for i in 1..n {
        let by2 = ugly_nums[pos_2] * 2;
        let by3 = ugly_nums[pos_3] * 3;
        let by5 = ugly_nums[pos_5] * 5;

        let minimum = by2.min(by3).min(by5);

        ugly_nums[i] = minimum;

        if minimum == by2 {
            pos_2 += 1;
        }
        if minimum == by3 {
            pos_3 += 1;
        }
        if minimum == by5 {
            pos_5 += 1;
        }
    }
    ugly_nums[n - 1]
}

#[test]
fn t1() {
    let tcases = [(10, 12)];
    for (n, expected) in tcases {
        assert_eq!(nth_ugly_number(n), expected);
    }
}
