// 2574. Left and Right Sum Differences
// https://leetcode.com/problems/left-and-right-sum-differences/

use crate::Solution;

impl Solution {
    pub fn left_rigth_difference(nums: Vec<i32>) -> Vec<i32> {
        let total_sum = nums.iter().sum::<i32>();
        let mut left_sum = 0;
        let mut result = Vec::with_capacity(nums.len());
        nums.into_iter().for_each(|x| {
            let right_sum = total_sum - x - left_sum;
            result.push((left_sum - right_sum).abs());
            left_sum += x;
        });
        result
    }

    pub fn left_rigth_difference_scan(nums: Vec<i32>) -> Vec<i32> {
        let mut result = nums
            .iter()
            .scan(0, |acc, &x| {
                *acc += x;
                Some(*acc)
            })
            .collect::<Vec<_>>();

        let mut right_sum = 0;
        result
            .iter_mut()
            .zip(nums.into_iter())
            .rev()
            .for_each(|(acc, x)| {
                *acc = (*acc - x - right_sum).abs();
                right_sum += x;
            });
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![15, 1, 11, 22],
            Solution::left_rigth_difference(vec![10, 4, 8, 3])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![0], Solution::left_rigth_difference(vec![1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![15, 1, 11, 22],
            Solution::left_rigth_difference_scan(vec![10, 4, 8, 3])
        );
    }
}
