// 45. Jump Game II
// https://leetcode.com/problems/jump-game-ii/description/

use crate::Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut dp = [10_000; 10_000];
        let n = nums.len();
        dp[n - 1] = 0;
        for (i, max_jump_len) in nums.into_iter().enumerate().rev().skip(1) {
            for jump in 1..=max_jump_len as usize {
                dp[i] = dp[i].min(1 + dp[(i + jump).min(n - 1)]);
            }
        }
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::jump(vec![2, 3, 1, 1, 4]));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::jump(vec![2, 3, 0, 1, 4]));
    }
}
