// 1680. Concatenation of Consecutive Binary Numbers
// https://leetcode.com/problems/concatenation-of-consecutive-binary-numbers/

use crate::Solution;

type SumType = i64;
const MOD: SumType = 1_000_000_007;

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        (1..=n).fold(0_i64, |res, num| {
            ((res << (32 - num.leading_zeros())) + num as i64) % 1_000_000_007
        }) as _
    }

    pub fn concatenated_binary_slow(n: i32) -> i32 {
        let masks = (0..32).map(|x| 1 << x).collect::<Vec<_>>();
        let mut res = 0;

        for num in 1..=n {
            let ind = (31 - num.leading_zeros()) as usize;
            for &bit in masks[..=ind].iter().rev() {
                res = (res << 1) % 1_000_000_007 + ((num & bit) > 0) as i32;
            }
        }
        res
    }

    // https://leetcode.com/problems/concatenation-of-consecutive-binary-numbers/discuss/2613227/rust-one-liner-with-comments
    pub fn concatenated_binary_reduce(n: i32) -> i32 {
        (1..=n as SumType)
            .reduce(|acc, num| ((acc << (SumType::BITS - num.leading_zeros())) | num) % MOD)
            .unwrap() as _
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
