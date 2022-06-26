// 2319. Check if Matrix Is X-Matrix
// https://leetcode.com/problems/check-if-matrix-is-x-matrix/

use crate::Solution;

impl Solution {
    pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
        let n = grid.len();
        grid.iter().enumerate().all(|(i, row)| {
            row.iter()
                .enumerate()
                .all(|(j, &x)| match i == j || i == (n - j - 1) {
                    true => x > 0,
                    _ => x == 0,
                })
        })
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::check_x_matrix(vec![
            vec![2, 0, 0, 1],
            vec![0, 3, 1, 0],
            vec![0, 5, 2, 0],
            vec![4, 0, 0, 2]
        ]));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::check_x_matrix(vec![
            vec![5, 7, 0],
            vec![0, 3, 1],
            vec![0, 5, 0]
        ]));
    }
}
