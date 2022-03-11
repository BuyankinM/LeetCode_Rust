// 500. Keyboard Row
// https://leetcode.com/problems/keyboard-row/

use crate::Solution;

impl Solution {
    pub fn find_words_new(words: Vec<String>) -> Vec<String> {
        use std::collections::HashSet;
        let lines = [
            "qwertyuiop".chars().collect::<HashSet<_>>(),
            "asdfghjkl".chars().collect::<HashSet<_>>(),
            "zxcvbnm".chars().collect::<HashSet<_>>(),
        ];
        words
            .into_iter()
            .filter(|word| {
                let word_it = word.chars().map(|c| c.to_ascii_lowercase());
                lines
                    .iter()
                    .any(|line| word_it.clone().all(|c| line.contains(&c)))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec!["Alaska".to_string(), "Dad".to_string()],
            Solution::find_words_new(vec![
                "Hello".to_string(),
                "Alaska".to_string(),
                "Dad".to_string(),
                "Peace".to_string()
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec!["".to_string(); 0],
            Solution::find_words_new(vec!["omk".to_string()])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec!["adsdf".to_string(), "sfd".to_string()],
            Solution::find_words_new(vec!["adsdf".to_string(), "sfd".to_string()])
        );
    }
}
