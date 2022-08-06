/*
    191 - Number of 1 bits
    Time: O(log(n))
    Space: O(1)
*/
pub fn hammingWeight(mut n: u32) -> i32 {
    let mut count = 0;
    while n != 0 {
        if n & 0b1 != 0 {
            count += 1;
        }
        n >>= 1;
    }
    count
}

#[test]
fn t1() {
    let tcases = [(0b11111111111111111111111111111101, 31)];
    for (num, count) in tcases {
        assert_eq!(hammingWeight(num), count);
    }
}
