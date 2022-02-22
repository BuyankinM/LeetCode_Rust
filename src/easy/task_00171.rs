// 171. Excel Sheet Column Number
// https://leetcode.com/problems/excel-sheet-column-number/

use crate::Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        column_title
            .chars()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, c)| {
                acc + ((c as i32) - ('A' as i32) + 1) * 26i32.pow(i as u32)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::title_to_number("A".to_string()));
    }

    #[test]
    fn test_2() {
        assert_eq!(28, Solution::title_to_number("AB".to_string()));
    }

    #[test]
    fn test_3() {
        assert_eq!(701, Solution::title_to_number("ZY".to_string()));
    }
}
