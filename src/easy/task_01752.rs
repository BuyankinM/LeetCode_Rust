// 1752. Check if Array Is Sorted and Rotated
// https://leetcode.com/problems/check-if-array-is-sorted-and-rotated/

use crate::Solution;

impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }

        let mut is_rotated = false;

        for i in 1..nums.len() {
            if nums[i] < nums[i - 1] {
                if !is_rotated {
                    is_rotated = true;
                } else {
                    return false;
                }
            }
        }

        if is_rotated {
            nums[0] >= nums[nums.len() - 1]
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::check(vec![3, 4, 5, 1, 2]));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::check(vec![2, 1, 3, 4]));
    }

    #[test]
    fn test_3() {
        assert!(Solution::check(vec![1, 2, 3]));
    }

    #[test]
    fn test_4() {
        assert!(Solution::check(vec![1, 1, 1]));
    }

    #[test]
    fn test_5() {
        assert!(Solution::check(vec![2, 1]));
    }
}
