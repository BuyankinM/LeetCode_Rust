// 55. Jump Game
// https://leetcode.com/problems/jump-game/

use crate::Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut jump_pos = 0;
        for (ind, &val) in nums.iter().enumerate() {
            match ind > jump_pos {
                true => return false,
                _ if jump_pos >= nums.len() - 1 => return true,
                _ => jump_pos = jump_pos.max(ind + val as usize),
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::can_jump(vec![2, 3, 1, 1, 4]));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::can_jump(vec![3, 2, 1, 0, 4]));
    }
}
