/*
    393 - Valid UTF-8
    Time: O(n)
    Space: O(1)
*/
pub fn valid_utf8(data: Vec<i32>) -> bool {
    let n = data.len();

    let mut count = 0;
    for i in (0..n).rev() {
        if data[i] & 0b1000_0000 == 0 && count == 0 {
            continue;
        }
        if 0b1000_0000 ^ (data[i] & 0b1100_0000) == 0 {
            count += 1;
        } else {
            let mask = match count {
                1 => (0b1100_0000, 0b1110_0000),
                2 => (0b1110_0000, 0b1111_0000),
                3 => (0b1111_0000, 0b1111_1000),
                _ => return false,
            };
            if mask.0 ^ (data[i] & mask.1) == 0 {
                count = 0;
            } else {
                return false;
            }
        }
    }
    count == 0
}
