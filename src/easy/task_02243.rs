// 2243. Calculate Digit Sum of a String
// https://leetcode.com/problems/calculate-digit-sum-of-a-string/

use crate::Solution;

impl Solution {
    pub fn digit_sum(s: String, k: i32) -> String {
        let mut s = s;
        while s.len() > k as usize {
            s = s
                .chars()
                .collect::<Vec<_>>()
                .chunks(k as usize)
                .map(|chunk| {
                    chunk
                        .iter()
                        .map(|c| c.to_digit(10).unwrap())
                        .sum::<u32>()
                        .to_string()
                })
                .collect::<Vec<_>>()
                .join("");
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
            Solution::digit_sum("11111222223".to_string(), 3),
            "135".to_string()
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::digit_sum("00000000".to_string(), 3),
            "000".to_string()
        );
    }
}
