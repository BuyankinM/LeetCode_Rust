// 884. Uncommon Words from Two Sentences
// https://leetcode.com/problems/uncommon-words-from-two-sentences/

use crate::Solution;

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut map = std::collections::HashMap::new();
        s1.split(' ')
            .chain(s2.split(' '))
            .for_each(|s| *map.entry(s).or_insert(0) += 1);
        map.iter()
            .filter_map(|(key, val)| match *val == 1 {
                true => Some((*key).to_string()),
                false => None,
            })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let correct = vec!["sour".to_string(), "sweet".to_string()];
        let mut ans = Solution::uncommon_from_sentences(
            "this apple is sweet".to_string(),
            "this apple is sour".to_string(),
        );
        ans.sort_unstable();
        assert_eq!(correct, ans);
    }

    #[test]
    fn test_2() {
        let correct = vec!["banana".to_string()];
        let ans =
            Solution::uncommon_from_sentences("apple apple".to_string(), "banana".to_string());
        assert_eq!(correct, ans);
    }
}
