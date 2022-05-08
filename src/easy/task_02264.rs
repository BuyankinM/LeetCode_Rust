// 2264. Largest 3-Same-Digit Number in String
// https://leetcode.com/problems/largest-3-same-digit-number-in-string/

use crate::Solution;

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        num.chars()
            .zip(num.chars().skip(1))
            .zip(num.chars().skip(2))
            .filter_map(|((a, b), c)| {
                if a == b && b == c {
                    Some(format!("{}{}{}", a, b, c))
                } else {
                    None
                }
            })
            .max()
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::largest_good_integer("6777133339".to_string()),
            "777".to_string()
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::largest_good_integer("2300019".to_string()),
            "000".to_string()
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::largest_good_integer("42352338".to_string()),
            "".to_string()
        );
    }
}
