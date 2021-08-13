// 1897. Redistribute Characters to Make All Strings Equal
// https://leetcode.com/problems/redistribute-characters-to-make-all-strings-equal/

use crate::Solution;

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let l = words.len();
        words
            .into_iter()
            .flat_map(|s| s.into_bytes())
            .fold([0; 26], |mut counter, b| {
                counter[(b - b'a') as usize] += 1;
                counter
            })
            .iter()
            .all(|x| *x % l == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::make_equal(vec![
            "abc".to_owned(),
            "aabc".to_owned(),
            "bc".to_owned()
        ]));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::make_equal(vec!["ab".to_owned(), "a".to_owned()]));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::make_equal(vec!["a".to_owned(), "b".to_owned()]));
    }
}
