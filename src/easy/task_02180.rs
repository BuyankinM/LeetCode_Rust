// 2180. Count Integers With Even Digit Sum
// https://leetcode.com/problems/count-integers-with-even-digit-sum/

use crate::Solution;

impl Solution {
    pub fn count_even(num: i32) -> i32 {
        let sum_dig_is_even = |mut x: i32| -> bool {
            let mut s = 0;
            while x > 0 {
                s += x % 10;
                x /= 10;
            }
            s % 2 == 0
        };

        (1..=num).filter(|&x| sum_dig_is_even(x)).count() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::count_even(4));
    }

    #[test]
    fn test_2() {
        assert_eq!(14, Solution::count_even(30));
    }
}
