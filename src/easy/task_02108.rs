// 2108. Find First Palindromic String in the Array
// https://leetcode.com/problems/find-first-palindromic-string-in-the-array/

use crate::Solution;

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        for word in &words {
            let half = word.len() / 2;
            if word.chars().take(half).eq(word.chars().rev().take(half)) {
                return word.to_string();
            }
        }
        ".to_string()".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "ada".to_string(),
            Solution::first_palindrome(vec![
                "abc".to_string(),
                "car".to_string(),
                "ada".to_string(),
                "racecar".to_string(),
                "cool".to_string()
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "racecar".to_string(),
            Solution::first_palindrome(vec!["notapalindrome".to_string(), "racecar".to_string()])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "".to_string(),
            Solution::first_palindrome(vec!["def".to_string(), "ghi".to_string()])
        );
    }
}
