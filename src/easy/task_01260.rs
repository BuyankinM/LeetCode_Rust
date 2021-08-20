// 1260. Shift 2D Grid
// https://leetcode.com/problems/shift-2d-grid/

use crate::Solution;

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let rows = grid.len();
        let cols = grid[0].len();
        let grid_len = rows * cols;

        grid.into_iter()
            .flatten()
            .cycle()
            .skip(grid_len - (k as usize) % grid_len)
            .take(grid_len)
            .collect::<Vec<_>>()
            .chunks(cols)
            .map(|row| row.to_owned())
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![vec![9, 1, 2], vec![3, 4, 5], vec![6, 7, 8]],
            Solution::shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 1)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![
                vec![12, 0, 21, 13],
                vec![3, 8, 1, 9],
                vec![19, 7, 2, 5],
                vec![4, 6, 11, 10]
            ],
            Solution::shift_grid(
                vec![
                    vec![3, 8, 1, 9],
                    vec![19, 7, 2, 5],
                    vec![4, 6, 11, 10],
                    vec![12, 0, 21, 13]
                ],
                4
            )
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
            Solution::shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 9)
        );
    }
}
