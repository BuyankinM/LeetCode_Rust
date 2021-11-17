// 883. Projection Area of 3D Shapes
// https://leetcode.com/problems/projection-area-of-3d-shapes/

use crate::Solution;

impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut max_in_cols = vec![0; grid.len()];
        for row in &grid {
            let mut max_in_row = 0;
            for (col, &val) in row.iter().enumerate().filter(|(_, val)| **val > 0) {
                max_in_row = max_in_row.max(val);
                max_in_cols[col] = max_in_cols[col].max(val);
                res += 1;
            }
            res += max_in_row;
        }
        res + max_in_cols.iter().sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(17, Solution::projection_area(vec![vec![1, 2], vec![3, 4]]));
    }

    #[test]
    fn test_2() {
        assert_eq!(5, Solution::projection_area(vec![vec![2]]));
    }

    #[test]
    fn test_3() {
        assert_eq!(8, Solution::projection_area(vec![vec![1, 0], vec![0, 2]]));
    }

    #[test]
    fn test_4() {
        assert_eq!(
            14,
            Solution::projection_area(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]])
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            21,
            Solution::projection_area(vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]])
        );
    }
}
