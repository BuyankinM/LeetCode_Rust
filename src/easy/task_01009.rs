// 1009. Complement of Base 10 Integer
// https://leetcode.com/problems/complement-of-base-10-integer/

use crate::Solution;

impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        !n & ((n as u32).next_power_of_two() - 1).max(1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::bitwise_complement(5));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::bitwise_complement(0));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::bitwise_complement(1));
    }

    #[test]
    fn test_4() {
        assert_eq!(10760938, Solution::bitwise_complement(123456789));
    }
}
