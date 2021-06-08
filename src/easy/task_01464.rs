// 1464. Maximum Product of Two Elements in an Array
// https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array/

use crate::Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut max1, mut max2) = (0, 0);
        nums.iter().for_each(|&n| {
            if n > max2 {
                max1 = max2;
                max2 = n;
            } else if n > max1 {
                max1 = n;
            }
        });
        (max1 - 1) * (max2 - 1)
    }

    pub fn max_product_mem(nums: Vec<i32>) -> i32 {
        let (mut max1, mut max2) = (0, 0);
        nums.iter().for_each(|&n| {
            if n > max2 {
                max1 = std::mem::replace(&mut max2, n);
            } else if n > max1 {
                max1 = n;
            }
        });
        (max1 - 1) * (max2 - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(12, Solution::max_product(vec![3, 4, 5, 2]));
    }

    #[test]
    fn test_2() {
        assert_eq!(16, Solution::max_product_mem(vec![1, 5, 4, 5]));
    }

    #[test]
    fn test_3() {
        assert_eq!(12, Solution::max_product_mem(vec![3, 7]));
    }
}
