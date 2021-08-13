// 58. Length of Last Word
// https://leetcode.com/problems/length-of-last-word/

use crate::Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim_end()
            .chars()
            .rev()
            .take_while(|c| c.is_alphabetic())
            .count() as i32
    }

    pub fn length_of_last_word_splitw(s: String) -> i32 {
        s.split_ascii_whitespace()
            .last()
            .map(|s| s.len() as i32)
            .unwrap_or(0)
    }

    pub fn length_of_last_word_split(s: String) -> i32 {
        s.trim().split(' ').last().unwrap().chars().count() as i32
    }

    pub fn length_of_last_word_rsplit(s: String) -> i32 {
        s.trim().rsplit(char::is_whitespace).next().unwrap().len() as i32
    }

    pub fn length_of_last_word_last(s: String) -> i32 {
        s.split_whitespace().last().unwrap_or("").len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::length_of_last_word("Hello World".to_owned()));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::length_of_last_word(" ".to_owned()));
    }

    #[test]
    fn test_3() {
        assert_eq!(5, Solution::length_of_last_word("Hello World  ".to_owned()));
    }
}
