/*
    66 - Plus One
    Time: O(n)
    Space: O(1)
*/
pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    let mut carry = true;
    for digit in digits.iter_mut().rev() {
        *digit = if *digit == 9 {
            0
        } else {
            carry = false;
            *digit + 1
        };
        if !carry {
            break;
        }
    }
    if carry {
        digits.insert(0, 1);
    }
    digits
}
