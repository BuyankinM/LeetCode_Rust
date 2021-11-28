// 2085. Count Common Words With One Occurrence
// https://leetcode.com/problems/count-common-words-with-one-occurrence/

use crate::Solution;

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        use std::collections::HashMap;
        let (mut m1, mut m2) = (HashMap::new(), HashMap::new());
        words1.iter().for_each(|w| *m1.entry(w).or_insert(0) += 1);
        words2.iter().for_each(|w| {
            if let Some(v) = m1.get(w) {
                if v == &1 {
                    *m2.entry(w).or_insert(0) += 1;
                }
            }
        });
        m2.values().filter(|v| **v == 1).count() as _
    }

    pub fn count_words_one_hashmap(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let mut m = std::collections::HashMap::new();
        words1.iter().for_each(|w| *m.entry(w).or_insert(0) += 1);
        words2.iter().for_each(|w| {
            m.entry(w).and_modify(|v| {
                if *v < 2 {
                    *v -= 1
                }
            });
        });
        m.values().filter(|v| **v == 0).count() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            2,
            Solution::count_words(
                vec![
                    "leetcode".to_string(),
                    "is".to_string(),
                    "amazing".to_string(),
                    "as".to_string(),
                    "is".to_string()
                ],
                vec![
                    "amazing".to_string(),
                    "leetcode".to_string(),
                    "is".to_string()
                ]
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            0,
            Solution::count_words(
                vec!["b".to_string(), "bb".to_string(), "bbb".to_string()],
                vec!["a".to_string(), "aa".to_string(), "aaa".to_string()]
            )
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            1,
            Solution::count_words(
                vec!["a".to_string(), "ab".to_string()],
                vec![
                    "a".to_string(),
                    "a".to_string(),
                    "a".to_string(),
                    "ab".to_string()
                ]
            )
        );
    }
}
