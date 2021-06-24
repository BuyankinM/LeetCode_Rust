// 9. Palindrome Number
// https://leetcode.com/problems/palindrome-number/

use crate::Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x.is_negative() || (x != 0 && x % 10 == 0) {
            return false;
        }

        let x_str = x.to_string();
        x_str[..x_str.len() / 2]
            .chars()
            .zip(x_str[x_str.len() / 2..].chars().rev())
            .all(|(c1, c2)| c1 == c2)
    }

    pub fn is_palindrome_eq(x: i32) -> bool {
        let a = x.to_string();
        a.chars().rev().eq(a.chars())
    }

    pub fn is_palindrome_num_rev(mut x: i32) -> bool {
        if x.is_negative() || (x != 0 && x % 10 == 0) {
            return false;
        }

        let mut rev_num = 0;

        while x > rev_num {
            rev_num = rev_num * 10 + x % 10;
            x /= 10;
        }

        return x == rev_num || x == rev_num / 10; // 3773 => 37==37, 37573 => 37 == (375/10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::is_palindrome(121));
    }

    #[test]
    fn test_2() {
        assert_eq!(false, Solution::is_palindrome(-121));
    }

    #[test]
    fn test_3() {
        assert_eq!(false, Solution::is_palindrome(10));
    }

    #[test]
    fn test_4() {
        assert_eq!(true, Solution::is_palindrome(1001));
    }

    #[test]
    fn test_5() {
        assert_eq!(true, Solution::is_palindrome_eq(1001));
    }

    #[test]
    fn test_6() {
        assert_eq!(true, Solution::is_palindrome_num_rev(1001));
    }

    #[test]
    fn test_7() {
        assert_eq!(true, Solution::is_palindrome_num_rev(10201));
    }
}
