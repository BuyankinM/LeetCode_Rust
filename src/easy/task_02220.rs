// 2220. Minimum Bit Flips to Convert Number
// https://leetcode.com/problems/minimum-bit-flips-to-convert-number/

use crate::Solution;

impl Solution {
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        (start ^ goal).count_ones() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::min_bit_flips(10, 7));
    }

    #[test]
    fn test_2() {
        assert_eq!(3, Solution::min_bit_flips(3, 4));
    }
}
