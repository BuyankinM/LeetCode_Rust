// 2131. Longest Palindrome by Concatenating Two Letter Words
// https://leetcode.com/problems/longest-palindrome-by-concatenating-two-letter-words/

use crate::Solution;

impl Solution {
    pub fn longest_palindrome_2131(words: Vec<String>) -> i32 {
        use std::cmp::Ordering::{Equal, Greater, Less};
        use std::collections::HashMap;

        let get_key = |w: &str| w.bytes().fold(0, |acc, b| acc | 1 << (b - b'a'));
        let mut eq_pairs = HashMap::new();
        let mut diff_pairs = HashMap::new();

        words.iter().for_each(|w| match w[..1].cmp(&w[1..]) {
            Equal => *eq_pairs.entry(w).or_insert(0) += 1,
            Less => diff_pairs.entry(get_key(w)).or_insert((0, 0)).0 += 1,
            Greater => diff_pairs.entry(get_key(w)).or_insert((0, 0)).1 += 1,
        });

        let mut n = 0;

        // try to find odd counts to middle element
        if let Some(x) = eq_pairs.values_mut().find(|x| **x % 2 == 1) {
            *x -= 1;
            n += 1;
        }

        // count other pairs
        eq_pairs.into_values().for_each(|x| n += 2 * (x / 2));

        // count min equal counts
        diff_pairs
            .values()
            .for_each(|&(n1, n2)| n += 2 * n1.min(n2));

        2 * n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::longest_palindrome_2131(vec![
                "lc".to_string(),
                "cl".to_string(),
                "gg".to_string()
            ]),
            6
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::longest_palindrome_2131(vec![
                "ab".to_string(),
                "ty".to_string(),
                "yt".to_string(),
                "lc".to_string(),
                "cl".to_string(),
                "ab".to_string()
            ]),
            8
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::longest_palindrome_2131(vec![
                "cc".to_string(),
                "ll".to_string(),
                "xx".to_string()
            ]),
            2
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::longest_palindrome_2131(vec![
                "dd".to_string(),
                "aa".to_string(),
                "bb".to_string(),
                "dd".to_string(),
                "aa".to_string(),
                "dd".to_string(),
                "bb".to_string(),
                "dd".to_string(),
                "aa".to_string(),
                "cc".to_string(),
                "bb".to_string(),
                "cc".to_string(),
                "dd".to_string(),
                "cc".to_string()
            ]),
            22
        );
    }
}
