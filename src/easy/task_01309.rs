// 1309. Decrypt String from Alphabet to Integer Mapping
// https://leetcode.com/problems/decrypt-string-from-alphabet-to-integer-mapping/

use crate::Solution;

impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let mut res = Vec::with_capacity(s.len());
        let (mut k, mut sum_sym) = (0, 0);

        s.as_bytes().iter().rev().for_each(|&b| {
            match b {
                b'#' => k = 2,
                _ if k == 0 => sum_sym = b - b'0',
                _ => {
                    sum_sym += (b - b'0') * 10u8.pow(2 - k);
                    k -= 1;
                }
            };
            if k == 0 {
                res.push(sum_sym + b'a' - 1);
                sum_sym = 0;
            }
        });

        res.reverse();

        String::from_utf8(res).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "jkab".to_string(),
            Solution::freq_alphabets("10#11#12".to_string())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "abcdefghijklmnopqrstuvwxyz".to_string(),
            Solution::freq_alphabets(
                "12345678910#11#12#13#14#15#16#17#18#19#20#21#22#23#24#25#26#".to_string()
            )
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "acz".to_string(),
            Solution::freq_alphabets("1326#".to_string())
        );
    }
}
