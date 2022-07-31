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
    let expected = vec![0, 1, 1, 2, 4, 7, 13, 24, 44];
    let answer = (0..expected.len() as i32)
        .map(tribonacci)
        .collect::<Vec<_>>();
    println!("{:?}", &answer);
    assert_eq!(expected, answer);
}
