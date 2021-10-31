// 1030. Matrix Cells in Distance Order
// https://leetcode.com/problems/matrix-cells-in-distance-order/

use crate::Solution;

impl Solution {
    pub fn all_cells_dist_order(rows: i32, cols: i32, row_c: i32, col_c: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::with_capacity((rows * cols) as usize);
        for r in 0..rows {
            for c in 0..cols {
                res.push(vec![r, c]);
            }
        }
        res.sort_unstable_by_key(|v| (v[0] - row_c).abs() + (v[1] - col_c).abs());
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![vec![0, 0], vec![0, 1]],
            Solution::all_cells_dist_order(1, 2, 0, 0)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![vec![0, 1], vec![0, 0], vec![1, 1], vec![1, 0]],
            Solution::all_cells_dist_order(2, 2, 0, 1)
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![
                vec![1, 2],
                vec![0, 2],
                vec![1, 1],
                vec![0, 1],
                vec![1, 0],
                vec![0, 0]
            ],
            Solution::all_cells_dist_order(2, 3, 1, 2)
        );
    }
}
