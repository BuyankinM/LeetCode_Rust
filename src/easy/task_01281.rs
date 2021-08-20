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

    pub fn subtract_product_and_sum_iter(n: i32) -> i32 {
        fn digits(mut n: i32) -> impl Iterator<Item = i32> {
            std::iter::from_fn(move || match n == 0 {
                false => {
                    let digit = n % 10;
                    n /= 10;
                    Some(digit)
                }
                true => None,
            })
        }
        digits(n).product::<i32>() - digits(n).sum::<i32>()
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

    #[test]
    fn test_3() {
        assert_eq!(21, Solution::subtract_product_and_sum_iter(4421));
    }
}
