// 1342. Number of Steps to Reduce a Number to Zero
// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/

use crate::Solution;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        match num {
            0 => 0,
            _ => (num.count_zeros() - num.leading_zeros() + 2 * num.count_ones() - 1) as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(6, Solution::number_of_steps(14));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::number_of_steps(0));
    }

    #[test]
    fn test_3() {
        assert_eq!(12, Solution::number_of_steps(123));
    }

    #[test]
    fn test_4() {
        assert_eq!(4, Solution::number_of_steps(8));
    }
}
