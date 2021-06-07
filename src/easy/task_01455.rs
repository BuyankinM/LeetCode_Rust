// 1455. Check If a Word Occurs As a Prefix of Any Word in a Sentence
// https://leetcode.com/problems/check-if-a-word-occurs-as-a-prefix-of-any-word-in-a-sentence/

use crate::Solution;

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        for (word, num) in sentence.split_whitespace().zip(1i32..) {
            if word.starts_with(&search_word) {
                return num;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            4,
            Solution::is_prefix_of_word("i love eating burger".to_string(), "burg".to_string())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            2,
            Solution::is_prefix_of_word(
                "this problem is an easy problem".to_string(),
                "pro".to_string()
            )
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            -1,
            Solution::is_prefix_of_word("i am tired".to_string(), "you".to_string())
        );
    }
}
