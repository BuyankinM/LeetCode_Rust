// 292. Nim Game
// https://leetcode.com/problems/nim-game/

use crate::Solution;

impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::can_win_nim(1));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::can_win_nim(4));
    }

    #[test]
    fn test_3() {
        assert!(Solution::can_win_nim(9));
    }

    #[test]
    fn test_4() {
        assert!(!Solution::can_win_nim(8));
    }
}
