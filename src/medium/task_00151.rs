// 151. Reverse Words in a String
// https://leetcode.com/problems/reverse-words-in-a-string/

use crate::Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut res = String::with_capacity(s.len());
        for word in s.split_ascii_whitespace().rev() {
            res = res + word + " ";
        }
        res.trim_end().to_string()
    }

    pub fn reverse_words_oneline(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "world hello".to_owned(),
            Solution::reverse_words("  hello world  ".to_owned())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "blue is sky the".to_owned(),
            Solution::reverse_words("the sky is blue".to_owned())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "Alice Loves Bob".to_owned(),
            Solution::reverse_words_oneline("  Bob    Loves  Alice   ".to_owned())
        );
    }
}
