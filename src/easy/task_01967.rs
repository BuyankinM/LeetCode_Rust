// 1967. Number of Strings That Appear as Substrings in Word
// https://leetcode.com/problems/number-of-strings-that-appear-as-substrings-in-word/

use crate::Solution;

impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        patterns
            .iter()
            .filter(|&pattern| word.contains(pattern))
            .count() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            3,
            Solution::num_of_strings(
                vec![
                    "a".to_string(),
                    "abc".to_string(),
                    "bc".to_string(),
                    "d".to_string()
                ],
                "abc".to_string()
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            2,
            Solution::num_of_strings(
                vec!["a".to_string(), "b".to_string(), "c".to_string()],
                "aaaaabbbbb".to_string()
            )
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            3,
            Solution::num_of_strings(
                vec!["a".to_string(), "a".to_string(), "a".to_string()],
                "ab".to_string()
            )
        );
    }
}
