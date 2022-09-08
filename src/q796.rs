/*
    796 - Rotate String
    Time: O(n)
    Space: O(n)
*/
pub fn rotate_string(s: String, goal: String) -> bool {
    s.len() == goal.len()
        && (s.clone() + &s).as_str().contains(&goal)
}
