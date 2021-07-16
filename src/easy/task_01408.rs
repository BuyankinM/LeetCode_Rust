// 1408. String Matching in an Array
// https://leetcode.com/problems/string-matching-in-an-array/

use crate::Solution;

impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let s = words.join("_");
        words
            .into_iter()
            .filter(|word| s.matches(word).nth(1).is_some())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec!["as".to_owned(), "hero".to_owned()],
            Solution::string_matching(vec![
                "mass".to_owned(),
                "as".to_owned(),
                "hero".to_owned(),
                "superhero".to_owned()
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec!["et".to_owned(), "code".to_owned()],
            Solution::string_matching(vec![
                "leetcode".to_owned(),
                "et".to_owned(),
                "code".to_owned()
            ])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Vec::<String>::new(),
            Solution::string_matching(vec!["blue".to_owned(), "green".to_owned(), "bu".to_owned()])
        );
    }
}
