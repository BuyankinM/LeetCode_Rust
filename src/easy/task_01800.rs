// 1800. Maximum Ascending Subarray Sum
// https://leetcode.com/problems/maximum-ascending-subarray-sum/

use crate::Solution;

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut glob_max = nums[0];
        let mut acc_max = nums[0];

        for (i, &el) in nums.iter().enumerate().skip(1) {
            match el > nums[i - 1] {
                true => acc_max += el,
                false => acc_max = el,
            }
            glob_max = acc_max.max(glob_max);
        }
        glob_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(65, Solution::max_ascending_sum(vec![10, 20, 30, 5, 10, 50]));
    }

    #[test]
    fn test_2() {
        assert_eq!(150, Solution::max_ascending_sum(vec![10, 20, 30, 40, 50]));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            33,
            Solution::max_ascending_sum(vec![12, 17, 15, 13, 10, 11, 12])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(100, Solution::max_ascending_sum(vec![100, 10, 1]));
    }
}
