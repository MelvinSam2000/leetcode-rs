/*
    208 - Prefix Trie
    Note:
    Most simple kind of trie
    Can be optimized for memory by using [Box<TrieNode>; 26]
    instead of HashMap
*/
use std::collections::HashMap;

struct TrieNode {
    ch: u8,
    next: HashMap<u8, Box<TrieNode>>,
}

impl TrieNode {
    pub fn new(ch: u8) -> Self {
        Self {
            ch,
            next: HashMap::new(),
        }
    }
}

pub struct Trie {
    head: TrieNode,
}

impl Trie {
    const START_CHAR: u8 = b'^';
    const END_CHAR: u8 = b'$';

    pub fn new() -> Self {
        Self {
            head: TrieNode::new(Self::START_CHAR),
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut tmp = &mut self.head;
        for &ch in word
            .as_bytes()
            .iter()
            .chain(Some(Self::END_CHAR).iter())
        {
            tmp.next.entry(ch).or_insert_with(|| {
                Box::new(TrieNode::new(ch))
            });
            tmp = tmp.next.get_mut(&ch).unwrap();
        }
    }

    pub fn search(&self, word: String) -> bool {
        let mut tmp = &self.head;
        for ch in word.as_bytes() {
            match tmp.next.get(ch) {
                Some(node) => {
                    if node.ch == Self::END_CHAR {
                        return true;
                    } else if node.ch == *ch {
                        tmp = &**node;
                    } else {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }
        }
        tmp.next.contains_key(&Self::END_CHAR)
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut tmp = &self.head;
        for ch in prefix.as_bytes() {
            match tmp.next.get(ch) {
                Some(node) => {
                    if node.ch == Self::END_CHAR {
                        return true;
                    } else if node.ch == *ch {
                        tmp = &**node;
                    } else {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }
        }
        true
    }
}
