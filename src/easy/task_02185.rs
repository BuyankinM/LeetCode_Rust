// 2185. Counting Words With a Given Prefix
// https://leetcode.com/problems/counting-words-with-a-given-prefix/

use crate::Solution;

impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        words
            .iter()
            .filter(|word| word.starts_with(pref.as_str()))
            .count() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            2,
            Solution::prefix_count(
                vec![
                    "pay".to_string(),
                    "attention".to_string(),
                    "practice".to_string(),
                    "attend".to_string()
                ],
                "at".to_string()
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            0,
            Solution::prefix_count(
                vec![
                    "leetcode".to_string(),
                    "win".to_string(),
                    "loops".to_string(),
                    "success".to_string()
                ],
                "code".to_string()
            )
        );
    }
}
