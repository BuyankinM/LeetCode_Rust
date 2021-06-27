// 1837. Sum of Digits in Base K
// https://leetcode.com/problems/sum-of-digits-in-base-k/

use crate::Solution;

impl Solution {
    pub fn sum_base(mut n: i32, k: i32) -> i32 {
        let mut res = 0;
        loop {
            res += n % k;
            n /= k;

            if n < k {
                break res + n;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(9, Solution::sum_base(34, 6));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::sum_base(10, 10));
    }
}
