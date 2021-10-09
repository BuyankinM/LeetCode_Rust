// 2027. Minimum Moves to Convert String
// https://leetcode.com/problems/minimum-moves-to-convert-string/

use crate::Solution;

impl Solution {
    pub fn minimum_moves(s: String) -> i32 {
        s.into_bytes()
            .iter()
            .enumerate()
            .filter(|&(_, b)| *b == b'X')
            .fold((0, 0), |(mut moves, mut next_ind), (ind, _)| {
                if ind >= next_ind {
                    moves += 1;
                    next_ind = ind + 3
                };
                (moves, next_ind)
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::minimum_moves("XXX".to_owned()));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::minimum_moves("XXOX".to_owned()));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::minimum_moves("OOOO".to_owned()));
    }
}
