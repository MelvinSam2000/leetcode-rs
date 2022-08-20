/*
    941 - Valid Mountain Array
    Time: O(n)
    Space: O(1)
*/
pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
    use std::cmp::Ordering;
    if arr.len() <= 2 || arr[0] >= arr[1] {
        return false;
    }

    let mut incr = true;
    for i in 0..arr.len() - 1 {
        if incr {
            match arr[i].cmp(&arr[i + 1]) {
                Ordering::Equal => return false,
                Ordering::Greater => incr = false,
                Ordering::Less => {}
            }
        } else if arr[i] <= arr[i + 1] {
            return false;
        }
    }
    !incr
}
