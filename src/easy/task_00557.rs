// 557. Reverse Words in a String III
// https://leetcode.com/problems/reverse-words-in-a-string-iii/

use crate::Solution;

impl Solution {
    pub fn reverse_words_557(s: String) -> String {
        let mut indicies = s
            .chars()
            .enumerate()
            .filter_map(|(i, c)| if c == ' ' { Some(i) } else { None })
            .collect::<Vec<_>>();
        indicies.push(s.len());

        let mut sb = s.into_bytes();
        let mut i = 0;
        for end in indicies {
            let mut j = end - 1;
            while i < j {
                sb.swap(i, j);
                i += 1;
                j -= 1;
            }
            i = end + 1;
        }
        String::from_utf8(sb).unwrap()
    }

    // https://leetcode.com/problems/reverse-words-in-a-string-iii/discuss/1170106/Rust-4ms
    pub fn reverse_words_short(s: String) -> String {
        s.split(' ')
            .map(|w| w.chars().rev().collect())
            .collect::<Vec<String>>()
            .join(" ")
    }

    // https://leetcode.com/problems/reverse-words-in-a-string-iii/discuss/1479260/rust-0ms-constant-space
    pub fn reverse_words_unsafe(mut s: String) -> String {
        assert!(s.is_ascii());
        unsafe { s.as_bytes_mut() }
            .split_mut(u8::is_ascii_whitespace)
            .for_each(<[_]>::reverse);
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "s'teL ekat edoCteeL tsetnoc".to_string(),
            Solution::reverse_words_557("Let's take LeetCode contest".to_string())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "doG gniD".to_string(),
            Solution::reverse_words_557("God Ding".to_string())
        );
    }
}
