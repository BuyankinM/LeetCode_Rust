// 1281. Subtract the Product and Sum of Digits of an Integer
// https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/

use crate::Solution;

impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let mut num = n;
        let (mut prod, mut sum) = (1, 0);
        while num > 0 {
            let d = num % 10;
            prod *= d;
            sum += d;
            num /= 10;
        }
        prod - sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(15, Solution::subtract_product_and_sum(234));
    }

    #[test]
    fn test_2() {
        assert_eq!(21, Solution::subtract_product_and_sum(4421));
    }
}
