// 1252. Cells with Odd Values in a Matrix
// https://leetcode.com/problems/cells-with-odd-values-in-a-matrix/

use crate::Solution;

impl Solution {
    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut rows = vec![0; m as usize];
        let mut cols = vec![0; n as usize];

        indices.iter().for_each(|cell| {
            rows[cell[0] as usize] += 1;
            cols[cell[1] as usize] += 1;
        });

        (0..m as usize).for_each(|row| {
            (0..n as usize).for_each(|col| {
                res += (rows[row] + cols[col]) % 2;
            })
        });

        res
    }

    pub fn odd_cells_bits(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        let (mut row_counter, mut col_counter) = (0_u64, 0_u64);

        indices.iter().for_each(|cell| {
            row_counter ^= 1 << cell[0];
            col_counter ^= 1 << cell[1];
        });

        let num_odd_rows = row_counter.count_ones() as i32;
        let num_odd_cols = col_counter.count_ones() as i32;

        num_odd_cols * m + num_odd_rows * n - 2 * num_odd_rows * num_odd_cols
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(6, Solution::odd_cells(2, 3, vec![vec![0, 1], vec![1, 1]]));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            6,
            Solution::odd_cells_bits(2, 3, vec![vec![0, 1], vec![1, 1]])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::odd_cells(2, 2, vec![vec![1, 1], vec![0, 0]]));
    }

    #[test]
    fn test_4() {
        assert_eq!(0, Solution::odd_cells_bits(2, 2, vec![vec![1, 1], vec![0, 0]]));
    }
}
