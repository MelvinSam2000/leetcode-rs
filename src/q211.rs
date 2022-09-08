/*
    211 - Design Add Search Word Data Structure
*/
struct TrieNode {
    ch: u8,
    next: [Option<Box<TrieNode>>; 27],
}

impl TrieNode {
    pub fn new(ch: u8) -> Self {
        Self {
            ch,
            next: Default::default(),
        }
    }
}

pub struct WordDictionary {
    head: TrieNode,
}

impl WordDictionary {
    const START_CHAR: u8 = b'^';
    const END_CHAR: u8 = b'$';
    const MATCH_CHAR: u8 = b'.';

    pub fn new() -> Self {
        Self {
            head: TrieNode::new(Self::START_CHAR),
        }
    }

    // Time: O(m), m is size of input word
    pub fn add_word(&mut self, word: String) {
        let mut word = word.as_bytes().to_vec();
        word.push(Self::END_CHAR);
        Self::add_word_helper(&mut self.head, &word);
    }

    // Time: O(m + n), m is size of input word, n is total # of nodes
    pub fn search(&self, word: String) -> bool {
        let mut word = word.as_bytes().to_vec();
        word.push(Self::END_CHAR);
        Self::search_word_helper(&self.head, &word)
    }

    fn add_word_helper(node: &mut TrieNode, word: &[u8]) {
        if word.is_empty() {
            return;
        }

        let val = match word[0] {
            b'$' => 26,
            x => (x - b'a') as usize,
        };
        if node.next[val].is_none() {
            node.next[val] =
                Some(Box::new(TrieNode::new(word[0])));
        }
        Self::add_word_helper(
            node.next[val].as_mut().unwrap(),
            &word[1..],
        )
    }

    fn search_word_helper(
        node: &TrieNode,
        word: &[u8],
    ) -> bool {
        if word.is_empty() {
            return false;
        }

        let children = node.next.iter().flatten();

        if word[0] == Self::MATCH_CHAR {
            for child in children {
                if Self::search_word_helper(
                    child,
                    &word[1..],
                ) {
                    return true;
                }
            }
        } else {
            for child in children {
                if child.ch == word[0]
                    && (child.ch == Self::END_CHAR
                        || Self::search_word_helper(
                            child,
                            &word[1..],
                        ))
                {
                    return true;
                }
            }
        }
        false
    }
}
