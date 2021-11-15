// 917. Reverse Only Letters
// https://leetcode.com/problems/reverse-only-letters/

use crate::Solution;

impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut sb = s.into_bytes();
        let (mut i, mut j) = (0, sb.len() - 1);
        while i < j {
            if !sb[i].is_ascii_alphabetic() {
                i += 1;
            } else if !sb[j].is_ascii_alphabetic() {
                j -= 1;
            } else {
                sb.swap(i, j);
                i += 1;
                j -= 1;
            }
        }
        String::from_utf8(sb).unwrap()
    }

    pub fn reverse_only_letters_func(s: String) -> String {
        let mut sb = s.into_bytes();
        let (mut i, mut j) = (0, sb.len() - 1);
        while i < j {
            match sb[i..].iter().position(|&b| b.is_ascii_alphabetic()) {
                Some(pos) => i += pos,
                _ => break,
            };

            match sb[i..j + 1]
                .iter()
                .rev()
                .position(|&b| b.is_ascii_alphabetic())
            {
                Some(pos) => j -= pos,
                _ => break,
            };

            sb.swap(i, j);
            i += 1;
            j -= 1;
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
            "dc-ba".to_string(),
            Solution::reverse_only_letters("ab-cd".to_string())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "j-Ih-gfE-dCba".to_string(),
            Solution::reverse_only_letters("a-bC-dEf-ghIj".to_string())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "Qedo1ct-eeLg=ntse-T!".to_string(),
            Solution::reverse_only_letters("Test1ng-Leet=code-Q!".to_string())
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            "Qedo1ct-eeLg=ntse-T!".to_string(),
            Solution::reverse_only_letters_func("Test1ng-Leet=code-Q!".to_string())
        );
    }
}
