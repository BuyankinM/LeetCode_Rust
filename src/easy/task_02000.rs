// 2000. Reverse Prefix of Word
// https://leetcode.com/problems/reverse-prefix-of-word/

use crate::Solution;

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let sb = word.as_bytes();
        match sb.iter().position(|&b| b == ch as u8) {
            Some(pos) => String::from_utf8(
                sb[..pos + 1]
                    .iter()
                    .rev()
                    .chain(sb[pos + 1..].iter())
                    .cloned()
                    .collect::<Vec<_>>(),
            )
            .unwrap(),
            None => word,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "dcbaefd".to_owned(),
            Solution::reverse_prefix("abcdefd".to_owned(), 'd')
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "zxyxxe".to_owned(),
            Solution::reverse_prefix("xyxzxe".to_owned(), 'z')
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "abcd".to_owned(),
            Solution::reverse_prefix("abcd".to_owned(), 'z')
        );
    }
}
