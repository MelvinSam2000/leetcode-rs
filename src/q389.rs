/*
    389 - Find the difference
    Time: O(n)
    Space: O(1)
*/
pub fn find_the_difference(s: String, t: String) -> char {
    let s = s.as_bytes();
    let t = t.as_bytes();

    let mut x = s.iter().fold(0, |prev, &y| prev ^ y);
    for &ch in t {
        x ^= ch;
    }
    x as char
}
