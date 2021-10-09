// 212. Word Search II
// https://leetcode.com/problems/word-search-ii/

use crate::Solution;
use std::collections::HashSet;

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    word: Option<String>,
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie: Trie = Default::default();
        let (m, n) = (board.len(), board[0].len());

        for word in words.iter() {
            let mut node = &mut trie;
            for c in word.as_bytes() {
                node =
                    node.children[(c - b'a') as usize].get_or_insert(Box::new(Default::default()));
            }
            node.word = Some(word.clone());
        }

        let mut answer: HashSet<String> = HashSet::new();
        for i in 0..m {
            for j in 0..n {
                let mut visited: Vec<Vec<bool>> = vec![vec![false; n]; m];
                Solution::dfs(&board, (i, j), &trie, &mut visited, &mut answer);
            }
        }
        answer.into_iter().collect()
    }

    fn dfs(
        board: &[Vec<char>],
        pos: (usize, usize),
        trie: &Trie,
        visited: &mut Vec<Vec<bool>>,
        answer: &mut HashSet<String>,
    ) {
        if visited[pos.0][pos.1] {
            return;
        }

        visited[pos.0][pos.1] = true;
        let c = board[pos.0][pos.1];

        if let Some(node) = &trie.children[(c as u8 - b'a') as usize] {
            if let Some(word) = &node.word {
                answer.insert(word.clone());
            }
            if pos.0 > 0 {
                Solution::dfs(board, (pos.0 - 1, pos.1), node.as_ref(), visited, answer);
            }
            if pos.1 > 0 {
                Solution::dfs(board, (pos.0, pos.1 - 1), node.as_ref(), visited, answer);
            }
            if pos.0 < board.len() - 1 {
                Solution::dfs(board, (pos.0 + 1, pos.1), node.as_ref(), visited, answer);
            }
            if pos.1 < board[0].len() - 1 {
                Solution::dfs(board, (pos.0, pos.1 + 1), node.as_ref(), visited, answer);
            }
        }
        visited[pos.0][pos.1] = false;
    }
}
