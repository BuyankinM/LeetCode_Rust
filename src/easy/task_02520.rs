// 2520. Count the Digits That Divide a Number
// https://leetcode.com/problems/count-the-digits-that-divide-a-number/

use crate::Solution;

impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let (mut n, mut res) = (num, 0);
        while n > 0 {
            res += (num % (n % 10) == 0) as i32;
            n /= 10;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::count_digits(7));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::count_digits(121));
    }
    #[test]
    fn test_3() {
        assert_eq!(4, Solution::count_digits(1248));
    }
}
