// 136. Single Number
// https://leetcode.com/problems/single-number/

use crate::Solution;

impl Solution {
    pub fn single_number_136(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |res, &x| res ^ x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::single_number_136(vec![2, 2, 1]));
    }

    #[test]
    fn test_2() {
        assert_eq!(4, Solution::single_number_136(vec![4, 1, 2, 1, 2]));
    }

    #[test]
    fn test_3() {
        assert_eq!(1, Solution::single_number_136(vec![1]));
    }
}
