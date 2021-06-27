// 1827. Minimum Operations to Make the Array Increasing
// https://leetcode.com/problems/minimum-operations-to-make-the-array-increasing/

use crate::Solution;

impl Solution {
    pub fn min_operations_increase(nums: Vec<i32>) -> i32 {
        let mut cur_max = nums[0];
        nums.iter().skip(1).fold(0, |mut res, &x| {
            cur_max = x.max(cur_max + 1);
            res += cur_max - x;
            res
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::min_operations_increase(vec![1, 1, 1]));
    }

    #[test]
    fn test_2() {
        assert_eq!(14, Solution::min_operations_increase(vec![1, 5, 2, 4, 1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::min_operations_increase(vec![8]));
    }
}
