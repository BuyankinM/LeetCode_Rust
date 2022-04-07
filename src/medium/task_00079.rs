// 79. Word Search
// https://leetcode.com/problems/word-search/

use crate::Solution;

impl Solution {
    const MOVES: [[i32; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        if board.is_empty() || board[0].is_empty() {
            return false;
        } else if word.is_empty() {
            return true;
        }

        let (m, n) = (board.len(), board[0].len());
        let word: Vec<char> = word.chars().collect();
        let mut used = vec![vec![false; n]; m];

        for i in 0..m {
            for j in 0..n {
                if Self::dfs_trie(&board, &word, &mut used, i, j) {
                    return true;
                }
            }
        }

        false
    }

    fn dfs_trie(
        board: &[Vec<char>],
        word: &[char],
        used: &mut Vec<Vec<bool>>,
        i: usize,
        j: usize,
    ) -> bool {
        if i >= board.len() || j >= board[0].len() || used[i][j] || board[i][j] != word[0] {
            return false;
        } else if word.len() == 1 {
            return true;
        }

        used[i][j] = true;
        for step in &Self::MOVES {
            let next_i = (i as i32 + step[0]) as usize;
            let next_j = (j as i32 + step[1]) as usize;
            if Self::dfs_trie(board, &word[1..], used, next_i, next_j) {
                return true;
            }
        }
        used[i][j] = false;

        false
    }
}
