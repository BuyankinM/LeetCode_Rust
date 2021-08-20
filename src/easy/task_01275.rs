// 1275. Find Winner on a Tic Tac Toe Game
// https://leetcode.com/problems/find-winner-on-a-tic-tac-toe-game/

use crate::Solution;

impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let mut table = [0; 9];
        moves
            .iter()
            .zip([1, 5].iter().cycle())
            .for_each(|(v, player)| {
                table[(3 * v[0] + v[1]) as usize] = *player;
            });

        for &(num_lines, start, start_step, cell_step) in
            [(3, 0, 3, 1), (3, 0, 1, 3), (1, 0, 0, 4), (1, 2, 0, 2)].iter()
        {
            for line in 0..num_lines {
                let mut sum_line = 0;
                let mut point = start + line * start_step;
                for _ in 0..3 {
                    sum_line += table[point as usize];
                    point += cell_step;
                }

                match sum_line {
                    3 => return "A".to_string(),
                    15 => return "B".to_string(),
                    _ => (),
                }
            }
        }

        match moves.len() {
            9 => "Draw".to_string(),
            _ => "Pending".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "A".to_string(),
            Solution::tictactoe(vec![
                vec![0, 0],
                vec![2, 0],
                vec![1, 1],
                vec![2, 1],
                vec![2, 2]
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "B".to_string(),
            Solution::tictactoe(vec![
                vec![0, 0],
                vec![1, 1],
                vec![0, 1],
                vec![0, 2],
                vec![1, 0],
                vec![2, 0]
            ])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "Draw".to_string(),
            Solution::tictactoe(vec![
                vec![0, 0],
                vec![1, 1],
                vec![2, 0],
                vec![1, 0],
                vec![1, 2],
                vec![2, 1],
                vec![0, 1],
                vec![0, 2],
                vec![2, 2]
            ])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            "Pending".to_string(),
            Solution::tictactoe(vec![vec![0, 0], vec![1, 1]])
        );
    }
}
