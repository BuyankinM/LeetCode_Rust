// 168. Excel Sheet Column Title
// https://leetcode.com/problems/excel-sheet-column-title/

use crate::Solution;

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut v = vec![];
        let mut n = column_number;
        while n > 0 {
            let c = (n - 1) % 26;
            v.push((b'A' + c as u8) as char);
            n = (n - 1) / 26;
        }
        v.iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::convert_to_title(1), "A".to_string());
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::convert_to_title(701), "ZY".to_string());
    }
}
