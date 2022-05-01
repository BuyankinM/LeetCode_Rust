// 2259. Remove Digit From Number to Maximize Result
// https://leetcode.com/problems/remove-digit-from-number-to-maximize-result/

use crate::Solution;

impl Solution {
    pub fn remove_digit(number: String, digit: char) -> String {
        let mut num = number + "a"; // additional char for catching last symbol
        let mut pos = 0;
        num.chars()
            .zip(num.chars().skip(1))
            .enumerate()
            .filter(|(_, (a, _))| a == &digit)
            .any(|(i, (a, b))| {
                pos = i;
                a < b
            });
        num.pop(); // drop additional char
        num.remove(pos);
        num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::remove_digit("123".to_string(), '3'),
            "12".to_string()
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::remove_digit("1231".to_string(), '1'),
            "231".to_string()
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::remove_digit("73197".to_string(), '7'),
            "7319".to_string()
        );
    }
}
