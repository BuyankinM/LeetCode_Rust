// 1175. Prime Arrangements
// https://leetcode.com/problems/prime-arrangements/

use crate::Solution;

const MOD: i64 = 1_000_000_007;
const PRIME: [i32; 25] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
];

impl Solution {
    pub fn num_prime_arrangements(n: i32) -> i32 {
        let factorial_mod = |n: i32| -> i64 { (2..=n as i64).fold(1, |acc, x| (acc * x) % MOD) };
        let num_primes = PRIME.iter().take_while(|&&x| x <= n).count() as i32;
        ((factorial_mod(num_primes) * factorial_mod(n - num_primes)) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(12, Solution::num_prime_arrangements(5));
    }

    #[test]
    fn test_2() {
        assert_eq!(261273600, Solution::num_prime_arrangements(15));
    }

    #[test]
    fn test_3() {
        assert_eq!(682289015, Solution::num_prime_arrangements(100));
    }
}
