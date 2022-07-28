/*
    42 - Trapping Rain Water
    Time: O(n)
    Space: O(1)
    Note: water_formula => water[i] = max(max(L[i], R[i]) - height[i], 0)
    where L[i] = max(height[..i]), R[i] = max(height[i+1..])
*/
pub fn trap(height: Vec<i32>) -> i32 {
    use std::cmp::max;

    let (mut l, mut r) = (0, height.len() - 1);
    let (mut maxl, mut maxr) = (height[l], height[r]);
    let mut total_water = 0;
    while l < r {
        if maxl <= maxr {
            total_water += max(maxl - height[l], 0);
            maxl = max(height[l], maxl);
        } else {
            total_water += max(maxr - height[r], 0);
            maxr = max(height[r], maxr);
        }
        if maxl <= maxr {
            l += 1;
        } else {
            r -= 1;
        }
    }
    total_water
}

#[test]
fn t1() {
    let tcases = [
        (vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1], 6),
        (vec![4, 2, 0, 3, 2, 5], 9),
    ];
    for (param, expected) in tcases {
        assert_eq!(trap(param), expected);
    }
}
