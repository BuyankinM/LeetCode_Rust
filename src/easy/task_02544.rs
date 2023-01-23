// 2544. Alternating Digit Sum
// https://leetcode.com/problems/alternating-digit-sum/

use crate::Solution;

impl Solution {
    pub fn alternate_digit_sum(mut n: i32) -> i32 {
        let (mut s0, mut s1, mut sign) = (0, 0, 1);

        while n > 0 {
            let rem = n % 10;
            s0 += sign * rem;
            s1 += -sign * rem;

            sign = -sign;
            n /= 10;
        }

        match sign < 0 {
            true => s0,
            false => s1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::alternate_digit_sum(521));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::alternate_digit_sum(111));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::alternate_digit_sum(886996));
    }
}
