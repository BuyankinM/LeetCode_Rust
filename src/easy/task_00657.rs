// 657. Robot Return to Origin
// https://leetcode.com/problems/robot-return-to-origin/

use crate::Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let (mut LR, mut UD) = (0, 0);
        moves.chars().for_each(|c| match c {
            'U' => UD += 1,
            'D' => UD -= 1,
            'R' => LR += 1,
            _ => LR -= 1,
        });
        LR == 0 && UD == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::judge_circle("UD".to_string()));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::judge_circle("LL".to_string()));
    }

    #[test]
    fn test_3() {
        assert!(Solution::judge_circle("UDLR".to_string()));
    }
}
