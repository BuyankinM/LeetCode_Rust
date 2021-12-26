// 2119. A Number After a Double Reversal
// https://leetcode.com/problems/a-number-after-a-double-reversal/

use crate::Solution;

impl Solution {
    pub fn is_same_after_reversals(num: i32) -> bool {
        num == 0 || num % 10 > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_same_after_reversals(526));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::is_same_after_reversals(5260));
    }

    #[test]
    fn test_3() {
        assert!(Solution::is_same_after_reversals(0));
    }
}
