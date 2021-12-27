// 693. Binary Number with Alternating Bits
// https://leetcode.com/problems/binary-number-with-alternating-bits/

use crate::Solution;

impl Solution {
    pub fn has_alternating_bits(mut n: i32) -> bool {
        let mut prev_bit = -1;
        while n > 0 {
            match n & 1 {
                bit if bit == prev_bit => return false,
                bit => prev_bit = bit,
            }
            n >>= 1;
        }
        true
    }

    pub fn has_alternating_bits_short(n: i32) -> bool {
        n <= 2 || n ^ (n >> 1) == ((n as usize).next_power_of_two() - 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::has_alternating_bits(5));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::has_alternating_bits(7));
    }

    #[test]
    fn test_3() {
        assert!(Solution::has_alternating_bits(1398101));
    }

    #[test]
    fn test_4() {
        assert!(!Solution::has_alternating_bits(447392426));
    }
}
