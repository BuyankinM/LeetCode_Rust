// 628. Maximum Product of Three Numbers
// https://leetcode.com/problems/maximum-product-of-three-numbers/

use crate::Solution;

impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        use std::mem::replace;

        let (mut max_1, mut max_2, mut max_3) = (i32::MIN, i32::MIN, i32::MIN);
        let (mut min_1, mut min_2, mut min_3) = (i32::MAX, i32::MAX, i32::MAX);
        let (mut num_pos, mut num_neg) = (0, 0);

        nums.iter().for_each(|&x| {
            match x.signum() {
                -1 => num_neg += 1,
                _ => num_pos += 1,
            };
            if x > max_1 {
                max_3 = max_2;
                max_2 = replace(&mut max_1, x);
            } else if x > max_2 {
                max_3 = replace(&mut max_2, x);
            } else if x > max_3 {
                max_3 = x;
            }

            if x < min_1 {
                min_3 = min_2;
                min_2 = replace(&mut min_1, x);
            } else if x < min_2 {
                min_3 = replace(&mut min_2, x);
            } else if x < min_3 {
                min_3 = x;
            }
        });

        if num_pos == 0 || num_neg <= 1 {
            max_1 * max_2 * max_3
        } else if num_pos <= 2 {
            min_1 * min_2 * max_1
        } else {
            (min_1 * min_2 * max_1).max(max_1 * max_2 * max_3)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(6, Solution::maximum_product(vec![1, 2, 3]));
    }

    #[test]
    fn test_2() {
        assert_eq!(24, Solution::maximum_product(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_3() {
        assert_eq!(-6, Solution::maximum_product(vec![-1, -2, -3]));
    }

    #[test]
    fn test_4() {
        assert_eq!(600, Solution::maximum_product(vec![-1, -2, -3, 100]));
    }

    #[test]
    fn test_5() {
        assert_eq!(18, Solution::maximum_product(vec![-1, -2, -3, 1, 2, 3]));
    }
}
