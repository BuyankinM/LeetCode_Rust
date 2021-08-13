// 1909. Remove One Element to Make the Array Strictly Increasing
// https://leetcode.com/problems/remove-one-element-to-make-the-array-strictly-increasing/

use crate::Solution;

impl Solution {
    pub fn can_be_increasing(nums: Vec<i32>) -> bool {
        let (mut is_dropped, mut prev_min) = (false, nums[0]);
        for (i, &num) in nums.iter().enumerate().skip(1) {
            match num <= prev_min {
                true if is_dropped => return false,
                true => {
                    is_dropped = true;
                    if i == 1 || num > nums[i - 2] {
                        prev_min = num;
                    }
                }
                false => {
                    prev_min = num;
                }
            };
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::can_be_increasing(vec![1, 2, 10, 5, 7]));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::can_be_increasing(vec![2, 3, 1, 2]));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::can_be_increasing(vec![1, 1, 1]));
    }

    #[test]
    fn test_4() {
        assert!(Solution::can_be_increasing(vec![1, 2, 3]));
    }

    #[test]
    fn test_5() {
        assert!(Solution::can_be_increasing(vec![105, 924, 32, 968]));
    }

    #[test]
    fn test_6() {
        assert!(Solution::can_be_increasing(vec![1, 2, 10, 2, 15]));
    }
}
