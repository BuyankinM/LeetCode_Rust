// 2578. Split With Minimum Sum
// https://leetcode.com/problems/split-with-minimum-sum/

use crate::Solution;

impl Solution {
    pub fn split_num(mut num: i32) -> i32 {
        let mut digits = Vec::with_capacity(9);
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        digits.sort_unstable();

        digits
            .into_iter()
            .enumerate()
            .fold([0; 2], |mut acc, (i, x)| {
                acc[i % 2] = acc[i % 2] * 10 + x;
                acc
            })
            .iter()
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(59, Solution::split_num(4325))
    }

    #[test]
    fn test_2() {
        assert_eq!(75, Solution::split_num(687))
    }
}
