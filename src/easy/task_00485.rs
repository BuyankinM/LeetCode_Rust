// 485. Max Consecutive Ones
// https://leetcode.com/problems/max-consecutive-ones/

use crate::Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let (mut max_ones, mut num_ones) = (0, 0);

        for elem in nums.iter() {
            if *elem == 1 {
                num_ones += 1;
            } else if num_ones > 0 {
                max_ones = max_ones.max(num_ones);
                num_ones = 0;
            }
        }
        max_ones.max(num_ones)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            3,
            Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            2,
            Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            4,
            Solution::find_max_consecutive_ones(vec![1, 1, 1, 1, 0, 1])
        );
    }
}
