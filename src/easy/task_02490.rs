// 2490. Circular Sentence
// https://leetcode.com/problems/circular-sentence/

use crate::Solution;

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let words = sentence.split_ascii_whitespace();
        let mut chars = sentence.chars();
        chars.clone().next().unwrap() == chars.next_back().unwrap()
            && words
                .clone()
                .zip(words.skip(1))
                .all(|(w1, w2)| w1.ends_with(w2.chars().next().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_circular_sentence(
            "leetcode exercises sound delightful".to_string()
        ));
    }

    #[test]
    fn test_2() {
        assert!(Solution::is_circular_sentence("eetcode".to_string()));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::is_circular_sentence(
            "Leetcode is cool".to_string()
        ));
    }
}
