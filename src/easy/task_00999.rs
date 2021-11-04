// 999. Available Captures for Rook
// https://leetcode.com/problems/available-captures-for-rook/

use crate::Solution;

impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        fn calc_cap(it: impl Iterator<Item = char>, rook_pos: usize) -> i32 {
            let (mut res, mut num) = (0, 0);
            for (pos, fig) in it.enumerate() {
                match (fig, pos < rook_pos) {
                    ('R', _) => res += num,
                    ('p', true) => num = 1,
                    ('B', true) => num = 0,
                    ('p', false) => return res + 1,
                    ('B', false) => return res,
                    _ => (),
                }
            }
            res
        }

        for (y, row) in board.iter().enumerate() {
            if let Some(x) = row.iter().position(|c| *c == 'R') {
                let row_it = row.iter().cloned();
                let col_it = (0..8_usize).map(|i| board[i][x]);
                return calc_cap(row_it, x) + calc_cap(col_it, y);
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            3,
            Solution::num_rook_captures(vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'R', '.', '.', '.', 'p'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.']
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            0,
            Solution::num_rook_captures(vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
                vec!['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
                vec!['.', 'p', 'B', 'R', 'B', 'p', '.', '.'],
                vec!['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
                vec!['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.']
            ])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            3,
            Solution::num_rook_captures(vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['p', 'p', '.', 'R', '.', 'p', 'B', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.']
            ])
        );
    }
}
