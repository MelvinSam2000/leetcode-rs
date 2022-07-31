/*
    70 - Climbing Stairs
    Time: O(n)
    Space: O(1)
*/
pub fn climb_stairs(n: i32) -> i32 {
    let (mut x, mut y, mut z) = (1, 1, 1);
    for _ in 1..n {
        z = x + y;
        x = y;
        y = z;
    }
    z
}

#[test]
fn t1() {
    let expected = vec![1, 2, 3, 5, 8, 13, 21];
    let answer = (1..=expected.len() as i32)
        .map(climb_stairs)
        .collect::<Vec<_>>();
    println!("{:?}", &answer);
    assert_eq!(expected, answer);
}
