// 1413. Minimum Value to Get Positive Step by Step Sum
// https://leetcode.com/problems/minimum-value-to-get-positive-step-by-step-sum/

use crate::Solution;

impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        match nums
            .iter()
            .fold((0, i32::MAX), |(acc, start_value), &x| {
                (acc + x, start_value.min(acc + x))
            })
            .1
        {
            min_val if min_val >= 1 => 1,
            min_val => 1 - min_val,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::min_start_value(vec![-3, 2, -3, 4, 2]));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::min_start_value(vec![1, 2]));
    }

    #[test]
    fn test_3() {
        assert_eq!(5, Solution::min_start_value(vec![1, -2, -3]));
    }

    #[test]
    fn test_4() {
        assert_eq!(1, Solution::min_start_value(vec![2, 2, 3]));
    }
}
