// 1512. Number of Good Pairs
// https://leetcode.com/problems/number-of-good-pairs/

use crate::Solution;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for (i, &a) in nums.iter().enumerate() {
            for &b in nums[i + 1..].iter() {
                if a == b {
                    res += 1;
                }
            }
        }
        res
    }

    pub fn num_identical_pairs_func(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold([0i32; 101], |mut acc, &x| {
                acc[x as usize] += 1;
                acc
            })
            .iter()
            .filter(|&&x| x > 1)
            .map(|&x| x * (x - 1) / 2)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]));
    }

    #[test]
    fn test_2() {
        assert_eq!(6, Solution::num_identical_pairs(vec![1, 1, 1, 1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::num_identical_pairs_func(vec![1, 2, 3]));
    }
}
