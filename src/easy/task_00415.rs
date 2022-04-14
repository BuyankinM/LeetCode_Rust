// 415. Add Strings
// https://leetcode.com/problems/add-strings/

use crate::Solution;

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut res = Vec::with_capacity(num1.len().max(num2.len()));
        let mut num1 = num1.chars().rev();
        let mut num2 = num2.chars().rev();
        let mut carry = 0;
        let to_digit = |c: char| c as u8 - b'0';

        loop {
            let sum = carry
                + match (num1.next(), num2.next()) {
                    (Some(d1), Some(d2)) => to_digit(d1) + to_digit(d2),
                    (Some(d1), None) => to_digit(d1),
                    (None, Some(d2)) => to_digit(d2),
                    (None, None) => {
                        if carry > 0 {
                            res.push((carry + b'0') as char);
                        }
                        break;
                    }
                };
            res.push((sum % 10 + b'0') as char);
            carry = sum / 10;
        }
        res.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::add_strings("123".to_string(), "456".to_string()),
            "579".to_string()
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::add_strings("123".to_string(), "0".to_string()),
            "123".to_string()
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::add_strings("1".to_string(), "9".to_string()),
            "10".to_string()
        );
    }
}
