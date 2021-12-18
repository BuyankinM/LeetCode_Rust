// 724. Find Pivot Index
// https://leetcode.com/problems/find-pivot-index/

use crate::Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let total_sum: i32 = nums.iter().sum();
        let mut left_sum = 0;
        for (i, &val) in nums.iter().enumerate() {
            if total_sum - left_sum - val == left_sum {
                return i as _;
            }
            left_sum += val;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]));
    }

    #[test]
    fn test_2() {
        assert_eq!(-1, Solution::pivot_index(vec![1, 2, 3]));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::pivot_index(vec![2, 1, -1]));
    }
}
