// 1913. Maximum Product Difference Between Two Pairs
// https://leetcode.com/problems/maximum-product-difference-between-two-pairs/

use std::f32::MIN;

use crate::Solution;

impl Solution {
    pub fn max_product_difference(mut nums: Vec<i32>) -> i32 {
        let l = nums.len();
        nums.sort_unstable();
        nums[l - 2] * nums[l - 1] - nums[0] * nums[1]
    }

    pub fn max_product_difference_linear(nums: Vec<i32>) -> i32 {
        let (mut min_1, mut min_2) = (i32::MAX, i32::MAX);
        let (mut max_1, mut max_2) = (i32::MIN, i32::MIN);

        nums.iter().for_each(|&n| {
            match n {
                _ if n > max_1 => max_2 = std::mem::replace(&mut max_1, n),
                _ if n > max_2 => max_2 = n,
                _ => (),
            };

            match n {
                _ if n < min_1 => min_2 = std::mem::replace(&mut min_1, n),
                _ if n < min_2 => min_2 = n,
                _ => (),
            };
        });
        
        max_1 * max_2 - min_1 * min_2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(34, Solution::max_product_difference(vec![5, 6, 2, 7, 4]));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            64,
            Solution::max_product_difference(vec![4, 2, 5, 9, 7, 4, 8])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            34,
            Solution::max_product_difference_linear(vec![5, 6, 2, 7, 4])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            64,
            Solution::max_product_difference_linear(vec![4, 2, 5, 9, 7, 4, 8])
        );
    }
}
