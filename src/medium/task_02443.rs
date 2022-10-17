// 2443. Sum of Number and Its Reverse
// https://leetcode.com/problems/sum-of-number-and-its-reverse/

use crate::Solution;

impl Solution {
    pub fn sum_of_number_and_reverse(num: i32) -> bool {
        let rev = |mut n: i32| {
            let mut res = 0;
            while n > 0 {
                res = 10 * res + n % 10;
                n /= 10;
            }
            res
        };
        (num / 2..=num).any(|n| n + rev(n) == num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::sum_of_number_and_reverse(443));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::sum_of_number_and_reverse(63));
    }

    #[test]
    fn test_3() {
        assert!(Solution::sum_of_number_and_reverse(181));
    }

    #[test]
    fn test_4() {
        assert!(Solution::sum_of_number_and_reverse(909));
    }
}
