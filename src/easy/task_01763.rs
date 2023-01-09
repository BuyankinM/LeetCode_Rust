// 1763. Longest Nice Substring
// https://leetcode.com/problems/longest-nice-substring/

use crate::Solution;

impl Solution {
    pub fn longest_nice_substring(s: String) -> String {
        let sb = s.as_bytes();
        let mut max_range: (usize, usize) = (0, 0);

        for i in 0..(s.len() - 1) {
            let (mut lower_mask, mut upper_mask) = (0u32, 0u32);

            for (j, &val) in sb.iter().enumerate().skip(i) {
                match val >= b'a' {
                    true => lower_mask |= 1 << (val - b'a'),
                    false => upper_mask |= 1 << (val - b'A'),
                };

                if lower_mask == upper_mask && (j + 1 - i) > (max_range.1 - max_range.0) {
                    max_range = (i, j + 1);
                }
            }
        }

        String::from_utf8(sb[max_range.0..max_range.1].to_vec()).unwrap()
    }

    pub fn longest_nice_substring_divide_and_conquer(s: String) -> String {
        use std::collections::HashSet;

        if s.len() < 2 {
            return "".to_string();
        }

        let chars: HashSet<char> = s.chars().collect();

        for (i, c) in s.chars().enumerate() {
            if chars.contains(&c.to_ascii_lowercase()) && chars.contains(&c.to_ascii_uppercase()) {
                continue;
            }
            let s1 = Self::longest_nice_substring(String::from(&s[0..i]));
            let s2 = Self::longest_nice_substring(String::from(&s[i + 1..]));
            if s1.len() >= s2.len() {
                return s1;
            } else {
                return s2;
            }
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "aAa".to_owned(),
            Solution::longest_nice_substring("YazaAay".to_owned())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "Bb".to_owned(),
            Solution::longest_nice_substring("Bb".to_owned())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "".to_owned(),
            Solution::longest_nice_substring("c".to_owned())
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            "dD".to_owned(),
            Solution::longest_nice_substring("dDzeE".to_owned())
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            "dD".to_owned(),
            Solution::longest_nice_substring_divide_and_conquer("dDzeE".to_owned())
        );
    }
}
