// 2299. Strong Password Checker II
// https://leetcode.com/problems/strong-password-checker-ii/

use crate::Solution;

impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        if password.len() < 8 {
            return false;
        }

        let mut res = 0;
        let mut prev_char = ' ';
        for c in password.chars() {
            match c == prev_char {
                true => return false,
                _ if c.is_ascii_lowercase() => res |= 0b1,
                _ if c.is_ascii_uppercase() => res |= 0b10,
                _ if c.is_ascii_digit() => res |= 0b100,
                _ => res |= 0b1000, // "!@#$%^&*()-+"
            }
            prev_char = c;
        }
        res == 0b1111
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::strong_password_checker_ii(
            "IloveLe3tcode!".to_string()
        ));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::strong_password_checker_ii(
            "Me+You--IsMyDream".to_string()
        ));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::strong_password_checker_ii("1aB!".to_string()));
    }
}
