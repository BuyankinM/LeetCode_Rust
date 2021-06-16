// 1523. Count Odd Numbers in an Interval Range
// https://leetcode.com/problems/count-odd-numbers-in-an-interval-range/

use crate::Solution;

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        (high - low) / 2 + (high % 2 | low % 2)
    }

    pub fn count_odds_bit_1(low: i32, high: i32) -> i32 {
        ((high - low) >> 1) + ((high & 1) | (low & 1))
    }

    pub fn count_odds_bit_2(low: i32, high: i32) -> i32 {
        ((high - low) >> 1) + ((high | low) & 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::count_odds(3, 7));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::count_odds_bit_1(8, 10));
    }

    #[test]
    fn test_3() {
        assert_eq!(2, Solution::count_odds_bit_2(7, 10));
    }
}
