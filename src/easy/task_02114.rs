// 2114. Maximum Number of Words Found in Sentences
// https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/

use crate::Solution;

impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        sentences.iter().fold(0, |acc, s| {
            acc.max(s.as_bytes().iter().filter(|&&b| b == b' ').count() + 1)
        }) as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            6,
            Solution::most_words_found(vec![
                "alice and bob love leetcode".to_string(),
                "i think so too".to_string(),
                "this is great thanks very much".to_string()
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            3,
            Solution::most_words_found(vec![
                "please wait".to_string(),
                "continue to fight".to_string(),
                "continue to win".to_string()
            ])
        );
    }
}
