// 2160. Minimum Sum of Four Digit Number After Splitting Digits
// https://leetcode.com/problems/minimum-sum-of-four-digit-number-after-splitting-digits/

use crate::Solution;

impl Solution {
    pub fn minimum_sum(mut num: i32) -> i32 {
        let mut digits = Vec::with_capacity(4);

        (0..4).for_each(|_| {
            digits.push(num % 10);
            num /= 10;
        });

        digits.sort_unstable();
        digits
            .into_iter()
            .enumerate()
            .fold(0, |sum, (i, x)| sum + x * if i < 2 { 10 } else { 1 })
    }

    pub fn minimum_sum_let(mut num: i32) -> i32 {
        let mut digits = Vec::with_capacity(4);

        (0..4).for_each(|_| {
            digits.push(num % 10);
            num /= 10;
        });

        digits.sort_unstable();

        if let &[d1, d2, d3, d4] = digits.as_slice() {
            return 10 * d1 + 10 * d2 + d3 + d4;
        }
        unreachable!();
    }

    pub fn minimum_sum_from_fn(mut num: i32) -> i32 {
        let mut digits = std::iter::from_fn(move || {
            let ost = num % 10;
            num /= 10;
            Some(ost)
        })
        .take(4)
        .collect::<Vec<_>>();

        digits.sort_unstable();

        if let &[d1, d2, d3, d4] = digits.as_slice() {
            return 10 * d1 + 10 * d2 + d3 + d4;
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(52, Solution::minimum_sum(2932));
    }

    #[test]
    fn test_2() {
        assert_eq!(13, Solution::minimum_sum(4009));
    }

    #[test]
    fn test_3() {
        assert_eq!(37, Solution::minimum_sum(1234));
    }
}
