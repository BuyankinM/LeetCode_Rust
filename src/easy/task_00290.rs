// 290. Word Pattern
// https://leetcode.com/problems/word-pattern/

use crate::Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        use std::collections::{HashMap, HashSet};

        if pattern.len() != s.matches(' ').count() + 1 {
            return false;
        }
        let mut hm = HashMap::new();
        let mut word_set = HashSet::new();

        for (word, c) in s.split_ascii_whitespace().zip(pattern.chars()) {
            if let Some(w) = hm.insert(c, word) {
                if w != word {
                    return false;
                }
            } else if !word_set.insert(word) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(!Solution::word_pattern(
            "abba".to_string(),
            "dog dog dog dog".to_string()
        ));
    }

    #[test]
    fn test_2() {
        assert!(Solution::word_pattern(
            "abba".to_string(),
            "dog cat cat dog".to_string()
        ));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::word_pattern(
            "abba".to_string(),
            "dog cat cat fish".to_string()
        ));
    }

    #[test]
    fn test_4() {
        assert!(!Solution::word_pattern(
            "aaa".to_string(),
            "aa aa aa aa".to_string()
        ));
    }
}
