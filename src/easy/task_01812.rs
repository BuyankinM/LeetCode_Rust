// 1812. Determine Color of a Chessboard Square
// https://leetcode.com/problems/determine-color-of-a-chessboard-square/

use crate::Solution;

impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        coordinates
            .as_bytes()
            .iter()
            .fold(0, |acc, &b| acc + (b - b'1') % 2)
            == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(false, Solution::square_is_white("a1".to_owned()));
    }

    #[test]
    fn test_2() {
        assert_eq!(true, Solution::square_is_white("h3".to_owned()));
    }

    #[test]
    fn test_3() {
        assert_eq!(false, Solution::square_is_white("c7".to_owned()));
    }
}
