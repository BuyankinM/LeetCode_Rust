// 1480. Running Sum of 1d Array
// https://leetcode.com/problems/running-sum-of-1d-array/

use crate::Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len()];
        res[0] = nums[0];

        for (i, &v) in nums.iter().enumerate().skip(1) {
            res[i] = res[i - 1] + v;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![1,3,6,10], Solution::running_sum(vec![1,2,3,4]));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![1,2,3,4,5], Solution::running_sum(vec![1,1,1,1,1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![3,4,6,16,17], Solution::running_sum(vec![3,1,2,10,1]));
    }
}

