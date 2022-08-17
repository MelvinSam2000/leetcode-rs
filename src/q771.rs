/*
    771 - Num of jewels in stones
    Time: O(n + m)
    Space: O(n)
*/
pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    use std::collections::HashSet;

    let jewels: HashSet<char> = HashSet::from_iter(jewels.chars());
    stones
        .chars()
        .filter(|stone| jewels.contains(stone))
        .count() as i32
}
