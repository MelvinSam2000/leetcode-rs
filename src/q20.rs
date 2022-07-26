/*
    20 - Valid Parentheses
    Time: O(n)
    Space: O(n)
*/
pub fn is_valid(s: String) -> bool {
    use std::collections::HashMap;
    let mut stack = Vec::new();
    let open = HashMap::from([(')', '('), (']', '['), ('}', '{')]);
    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' | ']' | '}' => {
                let x = *open.get(&ch).unwrap();
                match stack.pop() {
                    Some(y) => {
                        if x != y {
                            return false;
                        }
                    }
                    None => return false,
                }
            }
            _ => unreachable!(),
        }
    }
    stack.is_empty()
}
