// 541. Reverse String II
// https://leetcode.com/problems/reverse-string-ii/

use crate::Solution;

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let l = k as usize;
        let mut sb = s.into_bytes();

        for block in sb.chunks_mut(l * 2) {
            let mut i = 0;
            let mut j = l.min(block.len()) - 1;
            while i < j {
                block.swap(i, j);
                i += 1;
                j -= 1;
            }
        }
        String::from_utf8(sb).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "bacdfeg".to_string(),
            Solution::reverse_str("abcdefg".to_string(), 2)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "bacd".to_string(),
            Solution::reverse_str("abcd".to_string(), 2)
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "cbadefg".to_string(),
            Solution::reverse_str("abcdefg".to_string(), 3)
        );
    }
}
