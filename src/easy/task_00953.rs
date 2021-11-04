// 953. Verifying an Alien Dictionary
// https://leetcode.com/problems/verifying-an-alien-dictionary/

use crate::Solution;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let pos = |c: char| -> usize { (c as u8 - b'a') as usize };
        let mut ord = [0; 26];
        order.char_indices().for_each(|(ind, c)| ord[pos(c)] = ind);
        words
            .windows(2)
            .map(|pair| {
                let (w0, w1) = (&pair[0], &pair[1]);
                w1.chars()
                    .map(|c| ord[pos(c)])
                    .ge(w0.chars().map(|c| ord[pos(c)]))
            })
            .all(|x| x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_alien_sorted(
            vec!["hello".to_string(), "leetcode".to_string()],
            "hlabcdefgijkmnopqrstuvwxyz".to_string()
        ));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::is_alien_sorted(
            vec!["word".to_string(), "world".to_string(), "row".to_string()],
            "worldabcefghijkmnpqstuvxyz".to_string()
        ));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::is_alien_sorted(
            vec!["apple".to_string(), "app".to_string()],
            "abcdefghijklmnopqrstuvwxyz".to_string()
        ));
    }
}
