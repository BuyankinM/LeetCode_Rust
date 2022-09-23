// 1680. Concatenation of Consecutive Binary Numbers
// https://leetcode.com/problems/concatenation-of-consecutive-binary-numbers/

use crate::Solution;

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        (1..=n).fold(0_i64, |res, num| {
            (res << (32 - num.leading_zeros())) % 1_000_000_007 + num as i64
        }) as _
    }

    pub fn concatenated_binary_slow(n: i32) -> i32 {
        let masks = (0..32).into_iter().map(|x| 1 << x).collect::<Vec<_>>();
        let mut res = 0;

        for num in 1..=n {
            let ind = (31 - num.leading_zeros()) as usize;
            for &bit in masks[..=ind].iter().rev() {
                res = (res << 1) % 1_000_000_007 + ((num & bit) > 0) as i32;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::concatenated_binary(1), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::concatenated_binary(3), 27);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::concatenated_binary(12), 505379714);
    }
}
