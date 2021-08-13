// 1317. Convert Integer to the Sum of Two No-Zero Integers
// https://leetcode.com/problems/convert-integer-to-the-sum-of-two-no-zero-integers/

use crate::Solution;

impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        let test_zero_in = |mut x: i32| -> bool {
            while x > 0 {
                match x % 10 {
                    0 => return true,
                    _ => x /= 10,
                };
            }
            false
        };

        let mut a = 1;
        while test_zero_in(a) || test_zero_in(n - a) {
            a += 1;
        }
        vec![a, n - a]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![2, 9], Solution::get_no_zero_integers(11));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![1, 1], Solution::get_no_zero_integers(2));
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![11, 999], Solution::get_no_zero_integers(1010));
    }
}
