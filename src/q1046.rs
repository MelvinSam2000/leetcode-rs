/*
    1046 - Last Stone Weight
    Time: O(nlogn)
    Space: O(n)
*/
pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    use std::collections::BinaryHeap;
    let mut pq = BinaryHeap::from(stones);
    while pq.len() > 1 {
        let y = pq.pop().unwrap();
        let x = pq.pop().unwrap();
        if x != y {
            pq.push(y - x);
        }
    }
    pq.pop().unwrap_or(0)
}
