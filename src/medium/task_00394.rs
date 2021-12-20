// 394. Decode String
// https://leetcode.com/problems/decode-string/

use crate::Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        fn recursive(v: &[char]) -> (String, usize) {
            let mut s = String::new();
            let mut n = 0;
            let mut i = 0;
            while i < v.len() {
                match v[i] {
                    c if c.is_alphabetic() => s.push(c),
                    c if c.is_ascii_digit() => n = n * 10 + c.to_digit(10).unwrap(),
                    '[' => {
                        let (new_s, add_i) = recursive(&v[i + 1..]);
                        i += add_i;
                        s += &new_s.repeat(n as usize);
                        n = 0;
                    }
                    ']' => return (s, i + 1),
                    _ => panic!("Unexpected symbol"),
                };
                i += 1;
            }
            (s, 0)
        }

        let v = s.chars().collect::<Vec<_>>();
        recursive(&v).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "aaabcbc".to_string(),
            Solution::decode_string("3[a]2[bc]".to_string())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "accaccacc".to_string(),
            Solution::decode_string("3[a2[c]]".to_string())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "abcabccdcdcdef".to_string(),
            Solution::decode_string("2[abc]3[cd]ef".to_string())
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            "abccdcdcdxyz".to_string(),
            Solution::decode_string("abc3[cd]xyz".to_string())
        );
    }
}
