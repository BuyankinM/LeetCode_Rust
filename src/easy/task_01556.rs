// 1556. Thousand Separator
// https://leetcode.com/problems/thousand-separator/

use crate::Solution;

impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        n.to_string()
            .chars()
            .collect::<Vec<_>>()
            .rchunks(3)
            .rev()
            .map(|v| v.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join(".")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("987".to_owned(), Solution::thousand_separator(987));
    }

    #[test]
    fn test_2() {
        assert_eq!("1.234".to_owned(), Solution::thousand_separator(1234));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "123.456.789".to_owned(),
            Solution::thousand_separator(123456789)
        );
    }
}
