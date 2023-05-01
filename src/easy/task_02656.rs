// 2656. Maximum Sum With Exactly K Elements
// https://leetcode.com/problems/maximum-sum-with-exactly-k-elements/

use crate::Solution;

impl Solution {
    pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
        let max_el = nums.iter().max().unwrap();
        *max_el * k + (k - 1) * k / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(18, Solution::maximize_sum(vec![1, 2, 3, 4, 5], 3));
    }

    #[test]
    fn test_2() {
        assert_eq!(11, Solution::maximize_sum(vec![5, 5, 5], 2));
    }
}
