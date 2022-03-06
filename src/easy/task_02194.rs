// 2194. Cells in a Range on an Excel Sheet
// https://leetcode.com/problems/cells-in-a-range-on-an-excel-sheet/

use crate::Solution;

impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        let sb = s.as_bytes();
        (sb[0]..=sb[3])
            .flat_map(|col| {
                (sb[1]..=sb[4]).map(move |row| format!("{}{}", col as char, row as char))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![
                "K1".to_string(),
                "K2".to_string(),
                "L1".to_string(),
                "L2".to_string()
            ],
            Solution::cells_in_range("K1:L2".to_string())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![
                "A1".to_string(),
                "B1".to_string(),
                "C1".to_string(),
                "D1".to_string(),
                "E1".to_string(),
                "F1".to_string()
            ],
            Solution::cells_in_range("A1:F1".to_string())
        );
    }
}
