// 442. Find All Duplicates in an Array
// https://leetcode.com/problems/find-all-duplicates-in-an-array/

use crate::Solution;

impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());
        for i in 0..nums.len() {
            let abs_val = nums[i].abs();
            let ref_val = &mut nums[(abs_val - 1) as usize];
            match *ref_val < 0 {
                true => result.push(abs_val),
                false => *ref_val *= -1,
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![2, 3],
            Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![1], Solution::find_duplicates(vec![1, 1, 2]));
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![0i32; 0], Solution::find_duplicates(vec![1]));
    }
}
