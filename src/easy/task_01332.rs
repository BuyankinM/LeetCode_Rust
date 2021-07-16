// 1332. Remove Palindromic Subsequences
// https://leetcode.com/problems/remove-palindromic-subsequences/

use crate::Solution;

impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        if s.is_empty() {
            0
        } else if s.chars().eq(s.chars().rev()) {
            1
        } else {
            2
        }
    }

    pub fn remove_palindrome_sub_optimal(s: String) -> i32 {
        if s.is_empty() {
            0
        } else if s
            .chars()
            .take(s.len() / 2)
            .eq(s.chars().rev().take(s.len() / 2))
        {
            1
        } else {
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::remove_palindrome_sub("ababa".to_owned()));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::remove_palindrome_sub("abb".to_owned()));
    }

    #[test]
    fn test_3() {
        assert_eq!(2, Solution::remove_palindrome_sub("baabb".to_owned()));
    }
}
