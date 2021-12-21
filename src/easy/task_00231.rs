// 231. Power of Two
// https://leetcode.com/problems/power-of-two/

use crate::Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && n & (n - 1) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_power_of_two(1));
    }

    #[test]
    fn test_2() {
        assert!(Solution::is_power_of_two(4));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::is_power_of_two(0));
    }

    #[test]
    fn test_4() {
        assert!(!Solution::is_power_of_two(5));
    }

    #[test]
    fn test_5() {
        assert!(!Solution::is_power_of_two(-16));
    }
}
