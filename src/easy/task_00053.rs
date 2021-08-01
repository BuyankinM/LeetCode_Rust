// 53. Maximum Subarray
// https://leetcode.com/problems/maximum-subarray/

use crate::Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut max_sum, mut sum) = (nums[0], nums[0]);
        nums[1..].iter().for_each(|&x| {
            match sum {
                _ if sum > 0 => sum += x,
                _ if x > sum => sum = x,
                _ => (),
            };

            if sum > max_sum {
                max_sum = sum;
            }
        });
        max_sum
    }

    pub fn max_sub_array_kadanes_algo(nums: Vec<i32>) -> i32 {
        // Kadane's algorithm
        nums.iter()
            .fold((0, std::i32::MIN), |(cur, mx), &num| {
                (std::cmp::max(0, cur + num), std::cmp::max(mx, cur + num))
            })
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            6,
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::max_sub_array(vec![1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            23,
            Solution::max_sub_array_kadanes_algo(vec![5, 4, -1, 7, 8])
        );
    }
}
