// 318. Maximum Product of Word Lengths
// https://leetcode.com/problems/maximum-product-of-word-lengths/

use crate::Solution;

impl Solution {
    pub fn max_product_medium_318(words: Vec<String>) -> i32 {
        let v = words
            .iter()
            .map(|w| (w.len(), w.bytes().fold(0, |acc, b| acc | 1 << (b - b'a'))))
            .collect::<Vec<_>>();
        let mut max_len = 0;
        for (i, &(l1, c1)) in v.iter().enumerate() {
            for &(l2, c2) in v[i + 1..].iter() {
                if c1 & c2 == 0 {
                    max_len = max_len.max(l1 * l2);
                }
            }
        }
        max_len as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            16,
            Solution::max_product_medium_318(vec![
                "abcw".to_string(),
                "baz".to_string(),
                "foo".to_string(),
                "bar".to_string(),
                "xtfn".to_string(),
                "abcdef".to_string()
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            4,
            Solution::max_product_medium_318(vec![
                "a".to_string(),
                "ab".to_string(),
                "abc".to_string(),
                "d".to_string(),
                "cd".to_string(),
                "bcd".to_string(),
                "abcd".to_string()
            ])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            0,
            Solution::max_product_medium_318(vec![
                "a".to_string(),
                "aa".to_string(),
                "aaa".to_string(),
                "aaaa".to_string()
            ])
        );
    }
}
