// 1961. Check If String Is a Prefix of Array
// https://leetcode.com/problems/check-if-string-is-a-prefix-of-array/

use crate::Solution;

impl Solution {
    pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        let mut ls = s.len();
        let mut str_it = s.chars();

        for word in words.into_iter() {
            let lw = word.len();
            if ls < lw
                || !str_it
                    .by_ref()
                    .take(lw)
                    .zip(word.chars())
                    .all(|(c1, c2)| c1 == c2)
            {
                return false;
            } else if ls == lw {
                return true;
            }
            ls -= lw;
        }
        ls == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(
            Solution::is_prefix_string(
                "iloveleetcode".to_string(),
                vec![
                    "i".to_string(),
                    "love".to_string(),
                    "leetcode".to_string(),
                    "apples".to_string()
                ]
            )
        );
    }

    #[test]
    fn test_2() {
        assert!(!
            Solution::is_prefix_string(
                "iloveleetcode".to_string(),
                vec![
                    "apples".to_string(),
                    "i".to_string(),
                    "love".to_string(),
                    "leetcode".to_string(),
                ]
            )
        );
    }

    #[test]
    fn test_3() {
        assert!(!
            Solution::is_prefix_string(
                "iloveleetcode".to_string(),
                vec![
                    "i".to_string(),
                    "love".to_string(),
                    "leetcode!!!".to_string(),
                ]
            )
        );
    }

    #[test]
    fn test_4() {
        assert!(!
            Solution::is_prefix_string(
                "iloveleetcode!!!".to_string(),
                vec!["i".to_string(), "love".to_string(), "leetcode".to_string(),]
            )
        );
    }

    #[test]
    fn test_5() {
        assert!(!
            Solution::is_prefix_string(
                "iloveleet".to_string(),
                vec!["i".to_string(), "love".to_string(), "leetcode".to_string(),]
            )
        );
    }
}
