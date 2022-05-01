// 2255. Count Prefixes of a Given String
// https://leetcode.com/problems/count-prefixes-of-a-given-string/

use crate::Solution;

impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        words.iter().filter(|w| s.starts_with(w.as_str())).count() as _
    }

    pub fn count_prefixes_all(words: Vec<String>, s: String) -> i32 {
        words
            .iter()
            .filter(|w| w.len() <= s.len() && w.chars().zip(s.chars()).all(|(a, b)| a == b))
            .count() as _
    }

    pub fn count_prefixes_eq(words: Vec<String>, s: String) -> i32 {
        words
            .iter()
            .filter(|w| w.len() <= s.len() && w.as_str() == &s[..w.len()])
            .count() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::count_prefixes(
                vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "ab".to_string(),
                    "bc".to_string(),
                    "abc".to_string()
                ],
                "abc".to_string()
            ),
            3
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::count_prefixes(vec!["a".to_string(), "a".to_string()], "aa".to_string()),
            2
        );
    }
}
