/*
    190 - Reverse Bits
    Time: O(1)
    Space: O(1)
*/
pub fn reverse_bits(x: u32) -> u32 {
    let mut y = 0;
    for i in 0..32 {
        y |= ((x & (0b1 << i)) >> i) << (31 - i);
    }
    y
}
