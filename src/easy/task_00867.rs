// 867. Transpose Matrix
// https://leetcode.com/problems/transpose-matrix/

use crate::Solution;

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![0; matrix.len()]; matrix[0].len()];
        for (i, row) in matrix.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                res[j][i] = *val;
            }
        }
        res
    }

    // https://leetcode.com/problems/transpose-matrix/discuss/220986/Rust-4ms
    pub fn transpose_func(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n_row = a.len();
        let n_col = a.get(0).map(|row| row.len()).unwrap_or(0_usize);
        (0..n_col)
            .map(|c| (0..n_row).map(|r| a[r][c]).collect())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]],
            Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![vec![1, 4], vec![2, 5], vec![3, 6]],
            Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6]])
        );
    }
}
