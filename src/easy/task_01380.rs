// 1380. Lucky Numbers in a Matrix
// https://leetcode.com/problems/lucky-numbers-in-a-matrix/

use crate::Solution;

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::HashSet;

        let mut min_row = HashSet::with_capacity(matrix.len());
        let mut max_col = vec![i32::MIN; matrix[0].len()];

        for row in &matrix {
            let mut min_val = i32::MAX;
            for (col, &val) in row.iter().enumerate() {
                min_val = min_val.min(val);
                max_col[col] = max_col[col].max(val);
            }
            min_row.insert(min_val);
        }

        max_col
            .into_iter()
            .filter(|x| min_row.contains(x))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![15],
            Solution::lucky_numbers(vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![12],
            Solution::lucky_numbers(vec![
                vec![1, 10, 4, 2],
                vec![9, 3, 8, 7],
                vec![15, 16, 17, 12]
            ])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![7],
            Solution::lucky_numbers(vec![vec![7, 8], vec![1, 2]])
        );
    }
}
