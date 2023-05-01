// 2660. Determine the Winner of a Bowling Game
// https://leetcode.com/problems/determine-the-winner-of-a-bowling-game/

use crate::Solution;

use std::cmp::Ordering::{Greater, Less};

impl Solution {
    pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
        fn total_score(scores: Vec<i32>) -> i32 {
            let (mut total, mut double) = (0, 0);
            for score in scores {
                if double > 0 {
                    double -= 1;
                    total += score;
                }
                if score == 10 {
                    double = 2;
                }
                total += score;
            }
            total
        }

        let total_1 = total_score(player1);
        let total_2 = total_score(player2);
        match total_1.cmp(&total_2) {
            Greater => 1,
            Less => 2,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::is_winner(vec![4, 10, 7, 9], vec![6, 5, 2, 3]));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::is_winner(vec![3, 5, 7, 6], vec![8, 10, 10, 2]));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::is_winner(vec![2, 3], vec![4, 1]));
    }
}
