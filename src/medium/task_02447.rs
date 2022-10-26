// 2447. Number of Subarrays With GCD Equal to K
// https://leetcode.com/problems/number-of-subarrays-with-gcd-equal-to-k/

use crate::Solution;

impl Solution {
    pub fn subarray_gcd(nums: Vec<i32>, k: i32) -> i32 {
        use std::cmp::Ordering::{Equal, Less};
        fn gcd(a: i32, b: i32) -> i32 {
            match b > 0 {
                true => gcd(b, a % b),
                false => a,
            }
        }

        let mut res = 0;
        for (i, mut a) in nums.iter().cloned().enumerate().filter(|(_, a)| a % k == 0) {
            for &b in &nums[i..] {
                a = gcd(a, b);
                match a.cmp(&k) {
                    Equal => res += 1,
                    Less => break,
                    _ => (),
                }
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
        assert_eq!(4, Solution::subarray_gcd(vec![9, 3, 1, 2, 6, 3], 3));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::subarray_gcd(vec![4], 7));
    }
}
