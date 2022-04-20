// 326. Power of Three
// https://leetcode.com/problems/power-of-three/

use crate::Solution;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        n > 0 && 1_162_261_467 % n == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_power_of_three(3));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::is_power_of_three(6));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::is_power_of_three(0));
    }
}
