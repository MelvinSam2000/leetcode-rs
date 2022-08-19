/*
    205 - Isomorphic Strings
    Time: O(n)
    Space: O(n)
*/
pub fn is_isomorphic(s: String, t: String) -> bool {
    use std::collections::HashMap;

    if s.len() != t.len() {
        return false;
    }

    let mut ms = HashMap::new();
    let mut mt = HashMap::new();

    for (cs, ct) in s.chars().zip(t.chars()) {
        match (ms.get(&cs), mt.get(&ct)) {
            (None, None) => {
                ms.insert(cs, ct);
                mt.insert(ct, cs);
            }
            (Some(&xt), Some(&xs)) => {
                if cs != xs || ct != xt {
                    return false;
                }
            }
            _ => return false,
        }
    }

    true
}
