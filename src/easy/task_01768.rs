// 1768. Merge Strings Alternately
// https://leetcode.com/problems/merge-strings-alternately/

use crate::Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut res = String::new();
        let mut l1 = word1.len();
        let mut l2 = word2.len();
        let mut w1_iter = word1.chars();
        let mut w2_iter = word2.chars();

        while l1 > 0 || l2 > 0 {
            if l1 > 0 {
                res.push(w1_iter.next().unwrap());
                l1 -= 1;
            }
            if l2 > 0 {
                res.push(w2_iter.next().unwrap());
                l2 -= 1;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "apbqcr".to_owned(),
            Solution::merge_alternately("abc".to_owned(), "pqr".to_owned())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "apbqrs".to_owned(),
            Solution::merge_alternately("ab".to_owned(), "pqrs".to_owned())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "apbqcd".to_owned(),
            Solution::merge_alternately("abcd".to_owned(), "pq".to_owned())
        );
    }
}
