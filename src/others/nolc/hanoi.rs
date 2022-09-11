/*
    Lintcode 169 - Tower of Hanoi
    Time: O(2^n)
    Space: O(1) (O(n) counting recursion stack)
*/
pub fn tower_of_hanoi(n: i32) -> Vec<String> {
    helper(n, 'A', 'C', 'B')
}

fn helper(
    n: i32,
    src: char,
    dst: char,
    other: char,
) -> Vec<String> {
    if n == 0 {
        return vec![];
    }
    let mut step1 = helper(n - 1, src, other, dst);
    let mut step2 = vec![format!("from {src} to {dst}")];
    let mut step3 = helper(n - 1, other, dst, src);
    step1.append(&mut step2);
    step1.append(&mut step3);
    step1
}
