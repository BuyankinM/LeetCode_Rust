// 208. Implement Trie (Prefix Tree)
// https://leetcode.com/problems/implement-trie-prefix-tree/

use crate::Solution;

#[derive(Default)]
struct Trie {
    is_end: bool,
    nodes: [Option<Box<Trie>>; 26],
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut curr = self;
        for &b in word.as_bytes().iter() {
            curr = curr.nodes[(b - b'a') as usize].get_or_insert_with(|| Box::new(Trie::new()));
        }
        curr.is_end = true;
    }

    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        let mut curr = self;
        for &b in word.as_bytes().iter() {
            match curr.nodes[(b - b'a') as usize].as_ref() {
                Some(node) => curr = node,
                None => return false,
            }
        }
        curr.is_end
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        let mut curr = self;
        for &b in prefix.as_bytes().iter() {
            match curr.nodes[(b - b'a') as usize].as_ref() {
                Some(node) => curr = node,
                None => return false,
            }
        }
        true
    }
}
