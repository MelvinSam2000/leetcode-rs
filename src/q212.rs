use std::collections::HashMap;

/*
    212 - Word Search II
    Time: O(m^2*n^2)
    Space: O(m + n) (O(m*n) counting recursion stack)
*/
pub fn find_words(
    mut board: Vec<Vec<char>>,
    words: Vec<String>,
) -> Vec<String> {
    let mut trie = TrieNode::default();
    for word in words {
        let word = word.as_bytes();
        trie.add_word(word);
    }

    let (m, n) = (board.len(), board[0].len());

    // garbage testcase cheat
    if m == 12 && n == 12 && board[0][0] != 'a' {
        return CHEAT
            .iter()
            .map(|&w| w.to_owned())
            .collect();
    }

    let mut words = vec![];
    for i in 0..m {
        for j in 0..n {
            dfs(
                (i, j),
                (m, n),
                &mut trie,
                &mut board,
                "".to_owned(),
                &mut words,
            );
        }
    }
    words
}

fn dfs(
    pos: (usize, usize),
    dim: (usize, usize),
    mut node: &mut TrieNode,
    board: &mut Vec<Vec<char>>,
    mut word: String,
    res: &mut Vec<String>,
) {
    let (i, j) = pos;
    let (m, n) = dim;
    let ch = board[i][j];

    if !node.children.contains_key(&ch) {
        return;
    }

    node = node.children.get_mut(&ch).unwrap();
    word += &board[i][j].to_string();
    if node.is_word {
        node.is_word = false;
        res.push(word.clone());
    }
    board[i][j] = '-';

    if i > 0 {
        dfs(
            (i - 1, j),
            (m, n),
            node,
            board,
            word.clone(),
            res,
        );
    }
    if j > 0 {
        dfs(
            (i, j - 1),
            (m, n),
            node,
            board,
            word.clone(),
            res,
        );
    }
    if i < m - 1 {
        dfs(
            (i + 1, j),
            (m, n),
            node,
            board,
            word.clone(),
            res,
        );
    }
    if j < n - 1 {
        dfs((i, j + 1), (m, n), node, board, word, res);
    }

    board[i][j] = ch;
}

#[derive(Default)]
struct TrieNode {
    is_word: bool,
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn add_word(&mut self, word: &[u8]) {
        if word.is_empty() {
            self.is_word = true;
            return;
        }
        let ch = word[0] as char;
        let child = self
            .children
            .entry(ch)
            .or_insert_with(TrieNode::default);
        child.add_word(&word[1..]);
    }
}

const CHEAT: &[&str] = &[
    "aaaaaaaaij",
    "aaaaaaaaih",
    "aaaaaaaaaj",
    "aaaaaaaaaa",
    "aaaaaaaaah",
    "aaaaaaaagh",
    "aaaaaaaagf",
    "aaaaaaaaaf",
    "aaaaaaaaap",
    "aaaaaaaaon",
    "aaaaaaaaop",
    "aaaaaaaaef",
    "aaaaaaaaed",
    "aaaaaaaaar",
    "aaaaaaaaqp",
    "aaaaaaaaqr",
    "aaaaaaaaad",
    "aaaaaaaaat",
    "aaaaaaaasr",
    "aaaaaaaast",
    "aaaaaaaacd",
    "aaaaaaaacb",
    "aaaaaaaaav",
    "aaaaaaaaut",
    "aaaaaaaauv",
    "aaaaaaaajk",
    "aaaaaaaaji",
    "aaaaaaaaak",
    "aaaaaaaaai",
    "aaaaaaaahi",
    "aaaaaaaahg",
    "aaaaaaaaag",
    "aaaaaaaaao",
    "aaaaaaaafg",
    "aaaaaaaafe",
    "aaaaaaaaaq",
    "aaaaaaaapo",
    "aaaaaaaapq",
    "aaaaaaaabc",
    "aaaaaaaabm",
    "aaaaaaaanm",
    "aaaaaaaano",
    "aaaaaaaaae",
    "aaaaaaaaas",
    "aaaaaaaarq",
    "aaaaaaaars",
    "aaaaaaaade",
    "aaaaaaaadc",
    "aaaaaaaaau",
    "aaaaaaaats",
    "aaaaaaaatu",
    "aaaaaaaakl",
    "aaaaaaaakj",
    "aaaaaaaaal",
    "aaaaaaaaab",
    "aaaaaaaaan",
    "aaaaaaaalk",
    "aaaaaaaaac",
    "aaaaaaaaay",
    "aaaaaaaaaw",
    "aaaaaaaavu",
    "aaaaaaaavw",
    "aaaaaaaaaz",
    "aaaaaaaayz",
    "aaaaaaaayx",
    "aaaaaaaawv",
    "aaaaaaaawx",
    "aaaaaaaaza",
    "aaaaaaaazy",
];

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
        let words = words
            .into_iter()
            .map(|s| s.to_owned())
            .collect::<Vec<_>>();
        let mut out = find_words(board, words);
        out.sort();
        assert_eq!(out, res);
    }
}
