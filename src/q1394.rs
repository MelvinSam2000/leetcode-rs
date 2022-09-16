/*
    1394 - Lucky integer in array
    Time: O(n)
    Space: O(n)
*/
pub fn find_lucky(arr: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let freq = arr.into_iter().fold(
        HashMap::new(),
        |mut freq, elem| {
            freq.entry(elem)
                .and_modify(|c| *c += 1)
                .or_insert(1);
            freq
        },
    );
    let mut res = -1;
    for (elem, count) in freq {
        if count == elem && elem > res {
            res = elem;
        }
    }
    res
}
