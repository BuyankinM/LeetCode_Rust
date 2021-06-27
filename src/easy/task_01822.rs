// 1822. Sign of the Product of an Array
// https://leetcode.com/problems/sign-of-the-product-of-an-array/

use crate::Solution;

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        nums.iter().fold(1, |mut prod, &x| {
            prod *= x.signum();
            prod
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::array_sign(vec![-1, -2, -3, -4, 3, 2, 1]));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::array_sign(vec![1, 5, 0, 2, -3]));
    }

    #[test]
    fn test_3() {
        assert_eq!(-1, Solution::array_sign(vec![-1, 1, -1, 1, -1]));
    }
}
