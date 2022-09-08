use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
/*
    269 - Alien Dictionary
    n: Total characters from words added, m - Number of unique characters
    Time: O(n)
    Space: O(m)
*/
pub fn alien_dictionary(words: Vec<String>) -> String {
    // step 1: Create edge list of each character (O(n))
    let mut graph: HashMap<char, HashSet<char>> =
        HashMap::new();
    words.iter().for_each(|word| {
        word.chars().for_each(|ch| {
            graph.entry(ch).or_default();
        })
    });
    let mut invalid = false;
    words
        .windows(2)
        .map(|w| (w[0].as_bytes(), w[1].as_bytes()))
        .for_each(|(w0, w1)| {
            for (&ch1, &ch2) in w0.iter().zip(w1.iter()) {
                let ch1 = ch1 as char;
                let ch2 = ch2 as char;
                if ch1 != ch2 {
                    graph
                        .entry(ch1)
                        .and_modify(|set| {
                            set.insert(ch2);
                        })
                        .or_insert_with(|| {
                            HashSet::from_iter([ch2])
                        });
                    return;
                }
            }
            if w0.len() > w1.len() {
                invalid = true;
            }
        });
    if invalid {
        return "".to_owned();
    }
    // Step 2: Topological sort (Kahns algorithm)
    let mut indeg: HashMap<char, usize> =
        HashMap::from_iter(graph.keys().map(|&k| (k, 0)));
    for &u in graph.values().flatten() {
        indeg.entry(u).and_modify(|c| *c += 1).or_insert(1);
    }
    let mut q = BinaryHeap::new();
    for (&u, &deg) in indeg.iter() {
        if deg == 0 {
            q.push(Reverse(u));
        }
    }

    let mut out = vec![];
    while let Some(Reverse(v)) = q.pop() {
        out.push(v);
        for &u in graph.get(&v).into_iter().flatten() {
            let deg =
                indeg.get(&u).map(|&val| val - 1).unwrap();
            indeg.insert(u, deg);
            if deg == 0 {
                q.push(Reverse(u));
            }
        }
    }
    if indeg.values().any(|&c| c != 0) {
        return "".to_owned();
    }
    out.into_iter().collect()
}

#[test]
fn t1() {
    let tcases = [
        (vec!["wrt", "wrf", "er", "ett", "rftt"], "wertf"),
        (vec!["b", "ca", "cb", "dd", "de"], "abcde"),
        (vec!["baa", "abcd", "abca", "cab", "cad"], "bdac"),
        (vec!["caa", "aaa", "aab"], "cab"),
        (vec!["zy", "zx"], "yxz"),
        (vec!["ab", "adc"], "abcd"),
        (vec!["ab", "bb", "ba"], ""),
        (vec!["abc", "ab"], ""),
    ];
    for (i, (words, expected)) in
        tcases.into_iter().enumerate()
    {
        let words = words
            .into_iter()
            .map(|word| word.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            &alien_dictionary(words),
            expected,
            "Failed case {i}"
        );
    }
}
