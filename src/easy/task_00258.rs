// 258. Add Digits
// https://leetcode.com/problems/add-digits/

use crate::Solution;

impl Solution {
    pub fn add_digits(mut num: i32) -> i32 {
        while num > 9 {
            let (mut m, mut s) = (num, 0);
            while m > 0 {
                s += m % 10;
                m /= 10;
            }
            num = s;
        }
        num
    }

    // https://en.wikipedia.org/wiki/Digital_root
    pub fn add_digits_opt(num: i32) -> i32 {
        match num {
            0 => 0,
            _ => 1 + (num - 1) % 9,
        }
    }

    // https://leetcode.com/problems/add-digits/discuss/728092/Rust-Solutions
    pub fn add_digits_short(num: i32) -> i32 {
        (num - 1) % 9 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::add_digits(38));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::add_digits(10));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::add_digits(0));
    }
}
