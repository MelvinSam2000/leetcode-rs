/*
    1323 - Maximum 69 Number
    Time: O(logn)
    Space: O(1)
    Note: Haha funny sex number
*/
pub fn maximum69_number(mut num: i32) -> i32 {
    let mut index = None;
    let mut i = 0;
    let mut x = 0;
    while num != 0 {
        let digit = num % 10;
        if digit == 6 {
            index = Some(i);
        }
        x += digit * 10i32.pow(i);
        num /= 10;
        i += 1;
    }
    if let Some(index) = index {
        x += 3 * 10i32.pow(index);
    }
    x
}
