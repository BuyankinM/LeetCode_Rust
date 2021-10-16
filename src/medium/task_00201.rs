// 201. Bitwise AND of Numbers Range
// https://leetcode.com/problems/bitwise-and-of-numbers-range/

use crate::Solution;

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        (left..=right).fold(left, |acc, x| acc & x)
    }

    pub fn range_bitwise_and_pow2(left: i32, right: i32) -> i32 {
        if left == right {
            return right;
        }
        let right_prev_pow2 = ((right as u64).next_power_of_two() >> 1) as i32;
        (left.max(right_prev_pow2)..=right).fold(left, |acc, x| acc & x)
    }

    pub fn range_bitwise_and_short_cycle(left: i32, right: i32) -> i32 {
        let mut right = right;
        while left < right {
            right = right & (right - 1)
        }
        right
    }

    pub fn range_bitwise_and_shift(m: i32, n: i32) -> i32 {
        let (mut m, mut n) = (m, n);
        let mut i = 0;
        while n > m {
            n >>= 1;
            m >>= 1;
            i += 1;
        }
        m << i
    }

    pub fn range_bitwise_and_mask(m: i32, n: i32) -> i32 {
        let mut answer = 0;
        let mut mask = 0x4000_0000;
        for _ in 0..32 {
            if (m & mask) != (n & mask) {
                break;
            }
            answer += m & mask;
            mask >>= 1;
        }
        answer
    }

    pub fn range_bitwise_and_xor(m: i32, n: i32) -> i32 {
        m & !((1 << (32 - (m ^ n).leading_zeros())) - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::range_bitwise_and(5, 7));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::range_bitwise_and_short_cycle(1, 2147483647));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::range_bitwise_and_mask(1, 2147483647));
    }
}

