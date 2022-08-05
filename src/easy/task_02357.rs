// 2357. Make Array Zero by Subtracting Equal Amounts
// https://leetcode.com/problems/make-array-zero-by-subtracting-equal-amounts/

use crate::Solution;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        nums.dedup();
        (nums.len() - (nums[0] == 0) as usize) as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::minimum_operations(vec![1, 5, 0, 3, 5]), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::minimum_operations(vec![0]), 0);
    }
}
