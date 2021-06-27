// 1844. Replace All Digits with Characters
// https://leetcode.com/problems/replace-all-digits-with-characters/

use crate::Solution;

impl Solution {
    pub fn replace_digits(s: String) -> String {
        let mut prev_b = b'a';

        String::from_utf8(
            s.as_bytes()
                .iter()
                .map(|&cur_b| match cur_b >= b'a' {
                    true => {
                        prev_b = cur_b;
                        cur_b
                    }
                    false => prev_b + (cur_b - b'0'),
                })
                .collect(),
        )
        .unwrap()
    }

    pub fn replace_digits_unsafe(mut s: String) -> String {
        unsafe { s.as_bytes_mut() }
            .chunks_exact_mut(2)
            .for_each(|c| c[1] = c[0] + c[1] - b'0');
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "abcdef".to_owned(),
            Solution::replace_digits("a1c1e1".to_owned())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "abbdcfdhe".to_owned(),
            Solution::replace_digits_unsafe("a1b2c3d4e".to_owned())
        );
    }
}
