// 2523. Closest Prime Numbers in Range
// https://leetcode.com/problems/closest-prime-numbers-in-range/

use crate::Solution;

impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let start = 2.max(left) as usize;
        let end = right as usize;

        // Sieve of Eratosthenes
        let mut i = 2;
        let mut sieve = [true; 1_000_001];
        while i < 1 + (right as f64).sqrt() as usize {
            if sieve[i] {
                sieve[i * i..=end]
                    .iter_mut()
                    .step_by(i)
                    .for_each(|x| *x = false);
            }
            i += 1;
        }

        let mut prev_prime = -1;
        let mut first_prime = -1;
        let mut delta = i32::MAX;

        for i in sieve[start..=end]
            .iter()
            .zip(start..)
            .filter_map(|(x, i)| x.then(|| i as i32))
        {
            if prev_prime > -1 {
                let new_delta = i - prev_prime;
                if new_delta < delta {
                    first_prime = prev_prime;
                    delta = new_delta;
                    if delta <= 2 {
                        break;
                    }
                }
            }
            prev_prime = i;
        }

        match first_prime == -1 {
            true => vec![-1, -1],
            false => vec![first_prime, first_prime + delta],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![11, 13], Solution::closest_primes(10, 19));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![-1, -1], Solution::closest_primes(4, 6));
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![2, 3], Solution::closest_primes(1, 1_000_000));
    }
}
