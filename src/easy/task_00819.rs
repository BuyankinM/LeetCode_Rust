// 819. Most Common Word
// https://leetcode.com/problems/most-common-word/

use crate::Solution;

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        use std::collections::{HashMap, HashSet};

        let mut m = HashMap::new();
        let s = banned.iter().collect::<HashSet<_>>();

        let mut ans = String::new();
        let mut min_num = i32::MIN;

        for word in paragraph
            .split(&[' ', '!', '?', '\'', ',', ';', '.'][..])
            .map(|s| s.to_ascii_lowercase())
            .filter(|w| !w.is_empty() && !s.contains(w))
        {
            let num = m.entry(word.clone()).or_insert(0);
            *num += 1;
            if *num > min_num {
                min_num = *num;
                ans = word;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "ball".to_string(),
            Solution::most_common_word(
                "Bob hit a ball, the hit BALL flew far after it was hit.".to_string(),
                vec!["hit".to_string()]
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "a".to_string(),
            Solution::most_common_word("a.".to_string(), vec![])
        );
    }
}
