// 125. Valid Palindrome
// https://leetcode.com/problems/valid-palindrome/

use crate::Solution;

impl Solution {
    pub fn is_palindrome_filter(s: String) -> bool {
        let s_it = s.chars().filter_map(|c: char| match c.is_alphanumeric() {
            true => Some(c.to_ascii_lowercase()),
            false => None,
        });
        s_it.clone().eq(s_it.rev())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_palindrome_filter(
            "A man, a plan, a canal: Panama".to_string()
        ));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::is_palindrome_filter("race a car".to_string()));
    }

    #[test]
    fn test_3() {
        assert!(Solution::is_palindrome_filter("  ".to_string()));
    }
}
