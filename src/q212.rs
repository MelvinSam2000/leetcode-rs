use std::collections::HashMap;
use std::collections::HashSet;

/*
    212 - Word Search II
    Time: O(m^2*n^2)
    Space: O(m*n)
*/
pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    let trie = build_trie(&words);
    let m = board.len();
    let n = board[0].len();
    let mut visited = vec![vec![false; n]; m];
    let mut result = HashSet::new();
    for i in 0..m {
        for j in 0..n {
            for word in words.iter() {
                let w = word.as_bytes();
                if dfs(w, &trie, &board, &mut visited, (i, j), (m, n)) {
                    result.insert(word.clone());
                }
            }
        }
    }
    result.into_iter().collect()
}

fn build_trie(words: &Vec<String>) -> TrieNode {
    let mut trie = TrieNode::new(0);
    for word in words.iter() {
        let word = word.clone() + "$";
        trie.insert(word.as_bytes());
    }
    trie
}

fn dfs(
    word: &[u8],
    trie: &TrieNode,
    board: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    pos: (usize, usize),
    dim: (usize, usize),
) -> bool {
    if word.is_empty() {
        return false;
    }

    let (m, n) = dim;
    let (i, j) = pos;

    if board[i][j] != word[0] as char {
        return false;
    }

    if let Some(node) = trie.children.get(&word[0]) {
        if visited[i][j] {
            return false;
        }
        visited[i][j] = true;

        if i < m - 1 && dfs(&word[1..], node, board, visited, (i + 1, j), (m, n)) {
            visited[i][j] = false;
            return true;
        }

        if i > 0 && dfs(&word[1..], node, board, visited, (i - 1, j), (m, n)) {
            visited[i][j] = false;
            return true;
        }

        if j < n - 1 && dfs(&word[1..], node, board, visited, (i, j + 1), (m, n)) {
            visited[i][j] = false;
            return true;
        }

        if j > 0 && dfs(&word[1..], node, board, visited, (i, j - 1), (m, n)) {
            visited[i][j] = false;
            return true;
        }

        if word.len() == 1 {
            visited[i][j] = false;
            return node.children.contains_key(&b'$');
        }

        visited[i][j] = false;
    }
    false
}

#[derive(Debug)]
struct TrieNode {
    _ch: u8,
    children: HashMap<u8, Box<TrieNode>>,
}

impl TrieNode {
    fn new(_ch: u8) -> Self {
        Self {
            _ch,
            children: HashMap::new(),
        }
    }
}

impl TrieNode {
    fn insert(&mut self, word: &[u8]) {
        if word.is_empty() {
            return;
        }
        let ch = word[0];
        let node = self
            .children
            .entry(ch)
            .or_insert_with(|| Box::new(TrieNode::new(word[0])));
        Self::insert(node, &word[1..]);
    }
}

#[test]
fn t1() {
    let tcases = [
        (
            vec![
                vec!['o', 'a', 'a', 'n'],
                vec!['e', 't', 'a', 'e'],
                vec!['i', 'h', 'k', 'r'],
                vec!['i', 'f', 'l', 'v'],
            ],
            vec!["oath", "pea", "eat", "rain"],
            vec!["eat", "oath"],
        ),
        (
            vec![
                vec!['o', 'a', 'b', 'n'],
                vec!['o', 't', 'a', 'e'],
                vec!['a', 'h', 'k', 'r'],
                vec!['a', 'f', 'l', 'v'],
            ],
            vec!["oa", "oaa"],
            vec!["oa", "oaa"],
        ),
    ];

    for (board, words, res) in tcases {
        let words = words.into_iter().map(|s| s.to_owned()).collect::<Vec<_>>();
        let mut out = find_words(board, words);
        out.sort();
        assert_eq!(out, res);
    }
}
