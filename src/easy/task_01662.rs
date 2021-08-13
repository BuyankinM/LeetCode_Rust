// 1662. Check If Two String Arrays are Equivalent
// https://leetcode.com/problems/check-if-two-string-arrays-are-equivalent/

use crate::Solution;

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let wb_1: Vec<&u8> = word1.iter().map(|x| x.as_bytes()).flatten().collect();
        let wb_2: Vec<&u8> = word2.iter().map(|x| x.as_bytes()).flatten().collect();
        wb_1 == wb_2
    }

    pub fn array_strings_are_equal_join(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1.join("") == word2.join("")
    }

    pub fn array_strings_are_equal_concat(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1.concat() == word2.concat()
    }

    pub fn array_strings_are_equal_iter(word1: Vec<String>, word2: Vec<String>) -> bool {
        let s1: String = word1.into_iter().collect();
        let s2: String = word2.into_iter().collect();
        s1 == s2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::array_strings_are_equal(
            vec!["ab".to_owned(), "c".to_owned()],
            vec!["a".to_owned(), "bc".to_owned()]
        ));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::array_strings_are_equal_join(
            vec!["a".to_owned(), "cb".to_owned()],
            vec!["a".to_owned(), "bc".to_owned()]
        ));
    }

    #[test]
    fn test_3() {
        assert!(Solution::array_strings_are_equal_iter(
            vec!["abc".to_owned(), "d".to_owned(), "defg".to_owned()],
            vec!["abcddefg".to_owned()]
        ));
    }
}
