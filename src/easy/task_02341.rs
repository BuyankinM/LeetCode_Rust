// 2341. Maximum Number of Pairs in Array
// https://leetcode.com/problems/maximum-number-of-pairs-in-array/

use crate::Solution;

impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let mut counter = [0; 101];
        nums.iter().for_each(|&x| counter[x as usize] += 1);
        counter.into_iter().fold(vec![0, 0], |mut acc, x| {
            acc[0] += x / 2;
            acc[1] += x % 2;
            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::number_of_pairs(vec![1, 3, 2, 1, 3, 2, 2]),
            vec![3, 1],
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::number_of_pairs(vec![1, 1]), vec![1, 0]);
    }
    #[test]
    fn test_3() {
        assert_eq!(Solution::number_of_pairs(vec![0]), vec![0, 1]);
    }
}
