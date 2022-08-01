use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
/*
    269 - Alien Dictionary
    n: Total characters from words added, m - Number of unique characters
    Time: O(n)
    Space: O(m)
*/
pub fn alien_dictionary(words: Vec<&str>) -> String {
    let mut graph: HashMap<char, HashSet<char>> = HashMap::new();
    let mut out = VecDeque::new();
    let mut vertices = HashSet::new();
    // step 1: Create edge list of each character (O(n))
    words
        .windows(2)
        .map(|w| (w[0].chars(), w[1].chars()))
        .for_each(|(w0, w1)| {
            for (ch1, ch2) in w0.zip(w1) {
                vertices.insert(ch1);
                vertices.insert(ch2);
                if ch1 != ch2 {
                    match graph.get_mut(&ch1) {
                        Some(neighbours) => {
                            neighbours.insert(ch2);
                        }
                        None => {
                            let mut neighbours = HashSet::new();
                            neighbours.insert(ch2);
                            graph.insert(ch1, neighbours);
                        }
                    }
                    break;
                }
            }
        });
    // Step 2: Topological sort
    let n = graph.len();
    let mut visited = HashSet::new();
    while visited.len() < n {
        for &vertex in vertices.iter() {
            if !visited.contains(&vertex) {
                topsort(vertex, &graph, &mut visited, &mut out);
            }
        }
    }
    out.make_contiguous();
    out.as_slices().0.iter().collect()
}

fn topsort(
    src: char,
    graph: &HashMap<char, HashSet<char>>,
    visited: &mut HashSet<char>,
    out: &mut VecDeque<char>,
) {
    if visited.contains(&src) {
        return;
    }
    visited.insert(src);
    for &u in graph.get(&src).into_iter().flatten() {
        topsort(u, graph, visited, out);
    }
    out.push_front(src);
}

#[test]
fn t1() {
    let tcases = [
        (vec!["wrt", "wrf", "er", "ett", "rftt"], "wertf"),
        (vec!["b", "ca", "cb", "dd", "de"], "abcde"),
        (vec!["baa", "abcd", "abca", "cab", "cad"], "bdac"),
        (vec!["caa", "aaa", "aab"], "cab"),
    ];
    for (words, expected) in tcases {
        assert_eq!(&alien_dictionary(words), expected);
    }
}
