// 1903. Largest Odd Number in String
// https://leetcode.com/problems/largest-odd-number-in-string/

use crate::Solution;

impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        let max_odd_ind = num
            .char_indices()
            .filter(|(_, c)| (*c as u8) % 2 == 1) // '1' = 31, '3' = 33 ...
            .last();
        match max_odd_ind {
            None => "",
            Some((ind, _)) => &num[..=ind as usize],
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "5".to_owned(),
            Solution::largest_odd_number("52".to_owned())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "".to_owned(),
            Solution::largest_odd_number("4206".to_owned())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "35427".to_owned(),
            Solution::largest_odd_number("35427".to_owned())
        );
    }
}
