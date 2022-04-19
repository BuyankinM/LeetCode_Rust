// 342. Power of Four
// https://leetcode.com/problems/power-of-four/

use crate::Solution;

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        // check power of two => (n & (n-1)) == 0
        // check power of four => (n & 0x55555555) == n (0x55555555 = 1010101010101010101010101010101)
        n > 0 && (n & (n - 1)) == 0 && (n & 0x55555555) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_power_of_four(16));
    }

    #[test]
    fn test_2() {
        assert!(Solution::is_power_of_four(1));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::is_power_of_four(9999));
    }

    #[test]
    fn test_4() {
        assert!(!Solution::is_power_of_four(-9999));
    }
}
