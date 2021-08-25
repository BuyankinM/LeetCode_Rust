// 1979. Find Greatest Common Divisor of Array
// https://leetcode.com/problems/find-greatest-common-divisor-of-array/

use crate::Solution;

impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        fn gcd(a: i32, b: i32) -> i32 {
            match a == 0 {
                true => b,
                false => gcd(b % a, a),
            }
        }

        let (min_val, max_val) = nums
            .iter()
            .fold((i32::MAX, i32::MIN), |(min_val, max_val), &x| {
                (min_val.min(x), max_val.max(x))
            });

        gcd(max_val, min_val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::find_gcd(vec![2, 5, 6, 9, 10]));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::find_gcd(vec![7, 5, 6, 8, 3]));
    }

    #[test]
    fn test_3() {
        assert_eq!(3, Solution::find_gcd(vec![3, 3]));
    }
}
