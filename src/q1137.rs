/*
    1137 - Tribonacci Number
    Time: O(n)
    Space: O(1)
*/
pub fn tribonacci(n: i32) -> i32 {
    match n {
        0 | 1 => n,
        2 => 1,
        _ => {
            let (mut x, mut y, mut z, mut w) = (0, 1, 1, 0);
            for _ in 2..n {
                w = x + y + z;
                x = y;
                y = z;
                z = w;
            }
            w
        }
    }
}

#[test]
fn t1() {
    use itertools::assert_equal;
    let expected = [0, 1, 1, 2, 4, 7, 13, 24, 44].into_iter();
    let answer = (0..expected.len() as i32).into_iter().map(tribonacci);
    println!("{:?}", answer.clone().collect::<Vec<_>>());
    assert_equal(expected, answer);
}
