// 2540. Minimum Common Value
// https://leetcode.com/problems/minimum-common-value/

use crate::Solution;

impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut i = 0;
        for x in nums1 {
            if x > nums2[i] {
                while i < nums2.len() && x > nums2[i] {
                    i += 1;
                }
            }

            match i == nums2.len() {
                true => break,
                false if x == nums2[i] => return x,
                _ => (),
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::get_common(vec![1, 2, 3], vec![2, 4]));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::get_common(vec![1, 2, 3, 6], vec![2, 3, 4, 5]));
    }

    #[test]
    fn test_3() {
        assert_eq!(-1, Solution::get_common(vec![1, 2, 3, 6], vec![20, 30]));
    }
}
