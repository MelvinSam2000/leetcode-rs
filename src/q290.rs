/*
    290 - Word Pattern
    Time: O(n)
    Space: O(n)
    Note: Made the complicated way with traits, for fun
*/
pub fn word_pattern(pattern: String, s: String) -> bool {
    use std::hash::Hash;

    fn id_list<T, G>(list: T) -> Vec<usize>
    where
        T: IntoIterator<Item = G>,
        G: Hash + Eq,
    {
        use std::collections::HashMap;

        let mut dict = HashMap::new();

        list.into_iter().fold(vec![], |mut res, elem| {
            let id = dict.len();
            res.push(*dict.entry(elem).or_insert(id));
            res
        })
    }

    let p = pattern.chars().collect::<Vec<_>>();
    let s = s.split(' ').collect::<Vec<_>>();

    if p.len() != s.len() {
        return false;
    }

    id_list(p)
        .into_iter()
        .zip(id_list(s))
        .all(|(x, y)| x == y)
}

/*
    290 - Word Pattern
    Time: O(n)
    Space: O(n)
*/
pub fn word_pattern_v2(pattern: String, s: String) -> bool {
    use std::collections::HashMap;

    let s = s.split(' ').collect::<Vec<_>>();
    let pattern = pattern.chars().collect::<Vec<_>>();
    if pattern.len() != s.len() {
        return false;
    }

    let mut fwd = HashMap::new();
    let mut back = HashMap::new();

    for (ch, word) in pattern.iter().zip(s.iter()) {
        match fwd.get(&ch) {
            Some(w) => {
                if w != &word {
                    return false;
                }
            }
            None => {
                if back.contains_key(&word) {
                    return false;
                }
                fwd.insert(ch, word);
                back.insert(word, ch);
            }
        }
    }
    true
}
