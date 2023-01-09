// 17. Letter Combinations of a Phone Number
// https://leetcode.com/problems/letter-combinations-of-a-phone-number/

use crate::Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        fn backtrack(res: &mut Vec<String>, phone: &[&str], comb: String, next_digits: &str) {
            match next_digits.is_empty() {
                true => res.push(comb),
                false => {
                    let letters = phone[(next_digits.as_bytes()[0] - b'2') as usize];
                    letters.chars().for_each(|c| {
                        backtrack(res, phone, format!("{comb}{c}"), &next_digits[1..])
                    })
                }
            }
        }

        if digits.is_empty() {
            return vec![];
        }

        let mut res = Vec::new();
        let phone = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        let comb = String::new();
        backtrack(&mut res, &phone, comb, &digits);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            vec![
                "ad".to_string(),
                "ae".to_string(),
                "af".to_string(),
                "bd".to_string(),
                "be".to_string(),
                "bf".to_string(),
                "cd".to_string(),
                "ce".to_string(),
                "cf".to_string()
            ]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::letter_combinations("".to_string()),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::letter_combinations("2".to_string()),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
    }
}
