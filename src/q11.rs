/*
    11 - Container with most water
    Time: O(n)
    Space: O(1)
*/
pub fn max_area(height: Vec<i32>) -> i32 {
    use std::cmp::max;
    use std::cmp::min;
    let mut i = 0;
    let mut j = height.len() - 1;
    let mut max_area = 0;
    while i < j {
        let area = min(height[i], height[j]) * ((j - i) as i32);
        max_area = max(area, max_area);
        if height[i] < height[j] {
            i += 1;
        } else {
            j -= 1;
        }
    }
    max_area
}
