// 2529. Maximum Count of Positive Integer and Negative Integer
// https://leetcode.com/problems/maximum-count-of-positive-integer-and-negative-integer/

use crate::Solution;

impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let (num_neg, num_pos) = nums.iter().fold((0, 0), |(num_neg, num_pos), &x| {
            (num_neg + (x < 0) as i32, num_pos + (x > 0) as i32)
        });
        num_neg.max(num_pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::maximum_count(vec![-2, -1, -1, 1, 2, 3]));
    }

    #[test]
    fn test_2() {
        assert_eq!(3, Solution::maximum_count(vec![-3, -2, -1, 0, 0, 1, 2]));
    }
    
    #[test]
    fn test_3() {
        assert_eq!(4, Solution::maximum_count(vec![5, 20, 66, 1314]));
    }
}
