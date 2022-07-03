// 2325. Decode the Message
// https://leetcode.com/problems/decode-the-message/

use crate::Solution;

impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let idx = |b: u8| (b - b'a') as usize;
        let mut map = [0; 26];
        let mut i = 0;

        for b in key.bytes().filter(|&b| b != b' ') {
            match b {
                _ if i == 26 => break,
                _ if map[idx(b)] > 0 => continue,
                _ => map[idx(b)] = b'a' + i + 1,
            }
            i += 1;
        }

        let res = message
            .bytes()
            .map(|b| match b {
                b' ' => b' ',
                _ => map[idx(b)] - 1,
            })
            .collect();

        String::from_utf8(res).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "this is a secret".to_string(),
            Solution::decode_message(
                "the quick brown fox jumps over the lazy dog".to_string(),
                "vkbs bs t suepuv".to_string()
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "the five boxing wizards jump quickly".to_string(),
            Solution::decode_message(
                "eljuxhpwnyrdgtqkviszcfmabo".to_string(),
                "zwx hnfx lqantp mnoeius ycgk vcnjrdb".to_string()
            )
        );
    }
}
