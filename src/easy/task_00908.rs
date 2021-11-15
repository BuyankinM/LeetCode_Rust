// 908. Smallest Range I
// https://leetcode.com/problems/smallest-range-i/

use crate::Solution;

impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        let (min_val, max_val) = nums
            .iter()
            .fold((i32::MAX, i32::MIN), |(min_val, max_val), &x| {
                (min_val.min(x), max_val.max(x))
            });

        (max_val - min_val - 2 * k).max(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(0, Solution::smallest_range_i(vec![1], 0));
    }

    #[test]
    fn test_2() {
        assert_eq!(6, Solution::smallest_range_i(vec![0, 10], 2));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::smallest_range_i(vec![1, 3, 6], 3));
    }
}
