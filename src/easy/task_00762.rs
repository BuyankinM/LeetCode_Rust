// 762. Prime Number of Set Bits in Binary Representation
// https://leetcode.com/problems/prime-number-of-set-bits-in-binary-representation/

use crate::Solution;

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        use std::collections::HashSet;
        let primes = [2, 3, 5, 7, 11, 13, 17, 19].iter().collect::<HashSet<_>>();
        (left..=right)
            .filter(|&x| primes.contains(&(x.count_ones())))
            .count() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::count_prime_set_bits(6, 10));
    }

    #[test]
    fn test_2() {
        assert_eq!(5, Solution::count_prime_set_bits(10, 15));
    }
}
