/*
    1823 - Find winner of circle (Josephus Permutation)
    Time: O(n*k)
    Space: O(n)
    Note: Solution uses circular linked list represented with array
*/
pub fn find_the_winner(n: i32, k: i32) -> i32 {
    let n = n as usize;
    let mut next = (1..n).collect::<Vec<_>>();
    next.push(0);

    let mut slow = 0;
    let mut fast = n - 1;
    for _ in 0..n {
        for _ in 0..k {
            slow = fast;
            fast = next[fast];
        }
        next[slow] = next[fast];
    }
    fast as i32 + 1
}
