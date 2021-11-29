// 2089. Find Target Indices After Sorting Array
// https://leetcode.com/problems/find-target-indices-after-sorting-array/

use crate::Solution;

impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut num_target, mut num_before) = (0, 0);
        nums.iter()
            .filter(|x| **x <= target)
            .for_each(|&x| match x == target {
                true => num_target += 1,
                false => num_before += 1,
            });
        match num_target {
            0 => vec![],
            _ => (num_before..num_before + num_target).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![1, 2], Solution::target_indices(vec![1, 2, 5, 2, 3], 2));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![3], Solution::target_indices(vec![1, 2, 5, 2, 3], 3));
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![0; 0], Solution::target_indices(vec![1, 2, 5, 2, 3], 4));
    }
}
